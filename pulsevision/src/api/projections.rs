use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ProjectionParams {
    pub collective_id: Uuid,
}

#[derive(Serialize, Clone)]
pub struct Projection {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize)]
pub struct ProjectionsResponse {
    pub projections: Vec<Projection>,
    pub method: String,
    pub variance_explained: Vec<f32>,
    pub total_points: usize,
    pub embedding_dimension: usize,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/substrate/embeddings", get(get_projections))
}

/// Compute PCA projection from Nd to 3d.
///
/// Uses nalgebra SVD to find top 3 principal components.
/// Dimension is read from PulseDB at runtime — works with 384, 768, 1536, or custom.
pub fn pca_project(embeddings: &[(String, Vec<f32>)]) -> Result<(Vec<Projection>, Vec<f32>)> {
    if embeddings.is_empty() {
        return Ok((vec![], vec![]));
    }

    let n = embeddings.len();
    let d = embeddings[0].1.len();

    if d < 3 {
        return Err(Error::Projection(format!(
            "Embedding dimension {d} is less than 3, cannot project to 3D"
        )));
    }

    // Build N x D matrix
    let mut data = nalgebra::DMatrix::<f64>::zeros(n, d);
    for (i, (_, emb)) in embeddings.iter().enumerate() {
        for (j, &val) in emb.iter().enumerate() {
            data[(i, j)] = val as f64;
        }
    }

    // Center the data (subtract mean)
    let mean = data.row_mean();
    for i in 0..n {
        for j in 0..d {
            data[(i, j)] -= mean[j];
        }
    }

    // SVD — we only need the first 3 components
    let svd = nalgebra::linalg::SVD::new(data.clone(), true, true);

    let u = svd.u.ok_or_else(|| Error::Projection("SVD failed: no U matrix".into()))?;
    let singular_values = &svd.singular_values;

    // Variance explained by top 3 components
    let total_variance: f64 = singular_values.iter().map(|s| s * s).sum();
    let variance_explained: Vec<f32> = (0..3.min(singular_values.len()))
        .map(|i| {
            if total_variance > 0.0 {
                (singular_values[i] * singular_values[i] / total_variance) as f32
            } else {
                0.0
            }
        })
        .collect();

    // Project: take first 3 columns of U * S
    let projections: Vec<Projection> = embeddings
        .iter()
        .enumerate()
        .map(|(i, (id, _))| {
            let x = if !singular_values.is_empty() {
                (u[(i, 0)] * singular_values[0]) as f32
            } else {
                0.0
            };
            let y = if singular_values.len() > 1 {
                (u[(i, 1)] * singular_values[1]) as f32
            } else {
                0.0
            };
            let z = if singular_values.len() > 2 {
                (u[(i, 2)] * singular_values[2]) as f32
            } else {
                0.0
            };
            Projection {
                id: id.clone(),
                x,
                y,
                z,
            }
        })
        .collect();

    Ok((projections, variance_explained))
}

async fn get_projections(
    State(state): State<AppState>,
    Query(params): Query<ProjectionParams>,
) -> Result<Json<ProjectionsResponse>> {
    let collective = pulsedb::CollectiveId(params.collective_id);

    // Load all experiences with embeddings
    let experiences = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.list_experiences(collective, 10_000, 0)
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    let dim = state.substrate.embedding_dimension();

    // Extract id + embedding pairs
    let embeddings: Vec<(String, Vec<f32>)> = experiences
        .into_iter()
        .filter(|exp| !exp.embedding.is_empty())
        .map(|exp| (exp.id.to_string(), exp.embedding))
        .collect();

    // PCA in blocking thread
    let (projections, variance_explained) = tokio::task::spawn_blocking(move || {
        pca_project(&embeddings)
    })
    .await
    .map_err(|e| Error::Projection(e.to_string()))??;

    let total_points = projections.len();

    Ok(Json(ProjectionsResponse {
        projections,
        method: "pca".to_string(),
        variance_explained,
        total_points,
        embedding_dimension: dim,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pca_empty_input() {
        let (projections, variance) = pca_project(&[]).unwrap();
        assert!(projections.is_empty());
        assert!(variance.is_empty());
    }

    #[test]
    fn test_pca_single_point() {
        let embeddings = vec![("id1".to_string(), vec![1.0f32; 10])];
        let (projections, _) = pca_project(&embeddings).unwrap();
        assert_eq!(projections.len(), 1);
        assert_eq!(projections[0].id, "id1");
    }

    #[test]
    fn test_pca_multiple_points_384d() {
        let mut embeddings = Vec::new();
        for i in 0..20 {
            let mut emb = vec![0.0f32; 384];
            emb[i % 384] = 1.0;
            emb[(i * 7) % 384] = 0.5;
            embeddings.push((format!("id{i}"), emb));
        }
        let (projections, variance) = pca_project(&embeddings).unwrap();
        assert_eq!(projections.len(), 20);
        assert_eq!(variance.len(), 3);
        // Variance should sum to <= 1.0
        let total: f32 = variance.iter().sum();
        assert!(total <= 1.01); // Allow small floating point error
    }

    #[test]
    fn test_pca_768d() {
        let mut embeddings = Vec::new();
        for i in 0..10 {
            let mut emb = vec![0.0f32; 768];
            emb[i % 768] = 1.0;
            embeddings.push((format!("id{i}"), emb));
        }
        let (projections, _) = pca_project(&embeddings).unwrap();
        assert_eq!(projections.len(), 10);
    }

    #[test]
    fn test_pca_rejects_low_dimension() {
        let embeddings = vec![("id1".to_string(), vec![1.0f32, 2.0])];
        let result = pca_project(&embeddings);
        assert!(result.is_err());
    }

    #[test]
    fn test_pca_produces_3d_coordinates() {
        let mut embeddings = Vec::new();
        for i in 0..50 {
            let emb: Vec<f32> = (0..384).map(|j| ((i * 17 + j) as f32).sin()).collect();
            embeddings.push((format!("id{i}"), emb));
        }
        let (projections, _) = pca_project(&embeddings).unwrap();
        for p in &projections {
            assert!(p.x.is_finite());
            assert!(p.y.is_finite());
            assert!(p.z.is_finite());
        }
    }
}
