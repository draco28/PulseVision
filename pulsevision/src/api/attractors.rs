use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::projections::Projection;
use crate::error::{Error, Result};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AttractorParams {
    pub collective_id: Uuid,
    #[serde(default = "default_threshold")]
    pub threshold: f32,
}

fn default_threshold() -> f32 {
    0.5
}

#[derive(Serialize, Clone)]
pub struct Attractor {
    pub experience_id: String,
    pub position: Position3D,
    pub strength: f32,
    pub influence_radius: f32,
    pub warp_factor: f32,
    pub experience_type: String,
}

#[derive(Serialize, Clone)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize)]
pub struct AttractorsResponse {
    pub attractors: Vec<Attractor>,
    pub total: usize,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/substrate/attractors", get(get_attractors))
}

/// Compute attractor strength for an experience.
///
/// Formula: strength = importance * confidence * (1 + log(applications + 1))
pub fn compute_strength(importance: f32, confidence: f32, applications: u32) -> f32 {
    importance * confidence * (1.0 + (applications as f32 + 1.0).ln())
}

/// Compute influence radius proportional to strength.
pub fn compute_influence_radius(strength: f32) -> f32 {
    strength * 5.0
}

/// Compute warp factor (strength normalized to [0, 1]).
pub fn compute_warp_factor(strength: f32, max_strength: f32) -> f32 {
    if max_strength > 0.0 {
        (strength / max_strength).min(1.0)
    } else {
        0.0
    }
}

async fn get_attractors(
    State(state): State<AppState>,
    Query(params): Query<AttractorParams>,
) -> Result<Json<AttractorsResponse>> {
    let collective = pulsedb::CollectiveId(params.collective_id);
    let threshold = params.threshold;

    let experiences = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.list_experiences(collective, 10_000, 0)
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    // Compute PCA projections for positions
    let embeddings: Vec<(String, Vec<f32>)> = experiences
        .iter()
        .filter(|exp| !exp.embedding.is_empty())
        .map(|exp| (exp.id.to_string(), exp.embedding.clone()))
        .collect();

    let (projections, _) = tokio::task::spawn_blocking(move || {
        crate::api::projections::pca_project(&embeddings)
    })
    .await
    .map_err(|e| Error::Projection(e.to_string()))??;

    // Build projection lookup
    let proj_map: std::collections::HashMap<String, &Projection> =
        projections.iter().map(|p| (p.id.clone(), p)).collect();

    // Compute attractors
    let mut attractors: Vec<Attractor> = Vec::new();
    let mut max_strength: f32 = 0.0;

    for exp in &experiences {
        let strength = compute_strength(exp.importance, exp.confidence, exp.applications);
        if strength > threshold {
            max_strength = max_strength.max(strength);
            if let Some(proj) = proj_map.get(&exp.id.to_string()) {
                attractors.push(Attractor {
                    experience_id: exp.id.to_string(),
                    position: Position3D {
                        x: proj.x,
                        y: proj.y,
                        z: proj.z,
                    },
                    strength,
                    influence_radius: compute_influence_radius(strength),
                    warp_factor: 0.0, // Set below after max_strength is known
                    experience_type: format!("{:?}", exp.experience_type),
                });
            }
        }
    }

    // Normalize warp factors
    for attractor in &mut attractors {
        attractor.warp_factor = compute_warp_factor(attractor.strength, max_strength);
    }

    let total = attractors.len();
    Ok(Json(AttractorsResponse { attractors, total }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_strength() {
        let s = compute_strength(0.8, 0.9, 5);
        // 0.8 * 0.9 * (1 + ln(6)) ≈ 0.72 * 2.79 ≈ 2.01
        assert!(s > 1.5 && s < 2.5);
    }

    #[test]
    fn test_compute_strength_zero_applications() {
        let s = compute_strength(1.0, 1.0, 0);
        // 1.0 * 1.0 * (1 + ln(1)) = 1.0 * (1 + 0) = 1.0
        assert!((s - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_compute_strength_zero_importance() {
        let s = compute_strength(0.0, 1.0, 10);
        assert!((s - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_influence_radius() {
        let r = compute_influence_radius(2.0);
        assert!((r - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_warp_factor_normalization() {
        assert!((compute_warp_factor(1.0, 2.0) - 0.5).abs() < 0.001);
        assert!((compute_warp_factor(2.0, 2.0) - 1.0).abs() < 0.001);
        assert!((compute_warp_factor(0.0, 2.0) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_warp_factor_zero_max() {
        assert!((compute_warp_factor(1.0, 0.0) - 0.0).abs() < 0.001);
    }
}
