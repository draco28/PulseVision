use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListParams {
    pub collective_id: Uuid,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    100
}

#[derive(Serialize)]
pub struct ExperienceResponse {
    pub id: String,
    pub content_preview: String,
    pub experience_type: String,
    pub importance: f32,
    pub confidence: f32,
    pub applications: u32,
    pub domain: Vec<String>,
    pub timestamp_ms: i64,
    pub archived: bool,
}

#[derive(Serialize)]
pub struct ExperiencesListResponse {
    pub experiences: Vec<ExperienceResponse>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Serialize)]
pub struct ExperienceDetailResponse {
    pub id: String,
    pub content: String,
    pub experience_type: String,
    pub importance: f32,
    pub confidence: f32,
    pub applications: u32,
    pub domain: Vec<String>,
    pub related_files: Vec<String>,
    pub source_agent: String,
    pub timestamp_ms: i64,
}

#[derive(Serialize)]
pub struct RelationResponse {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relation_type: String,
    pub strength: f32,
}

#[derive(Serialize)]
pub struct RelationsListResponse {
    pub relations: Vec<RelationResponse>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct InsightResponse {
    pub id: String,
    pub content: String,
    pub insight_type: String,
    pub confidence: f32,
    pub source_count: usize,
    pub domain: Vec<String>,
}

#[derive(Serialize)]
pub struct InsightsListResponse {
    pub insights: Vec<InsightResponse>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct CollectiveResponse {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CollectivesListResponse {
    pub collectives: Vec<CollectiveResponse>,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub experience_count: usize,
    pub relation_count: usize,
    pub insight_count: usize,
    pub embedding_dimension: usize,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/substrate/experiences", get(list_experiences))
        .route("/api/substrate/experiences/{id}", get(get_experience))
        .route("/api/substrate/relations", get(list_relations))
        .route("/api/substrate/insights", get(list_insights))
        .route("/api/substrate/collectives", get(list_collectives))
        .route("/api/substrate/stats", get(get_stats))
}

async fn list_experiences(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<ExperiencesListResponse>> {
    let collective = pulsedb::CollectiveId(params.collective_id);
    let limit = params.limit.min(1000);

    let experiences = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.list_experiences(collective, limit, params.offset)
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    let response: Vec<ExperienceResponse> = experiences
        .iter()
        .map(|exp| ExperienceResponse {
            id: exp.id.to_string(),
            content_preview: exp.content.chars().take(200).collect(),
            experience_type: format!("{:?}", exp.experience_type),
            importance: exp.importance,
            confidence: exp.confidence,
            applications: exp.applications,
            domain: exp.domain.clone(),
            timestamp_ms: exp.timestamp.0,
            archived: exp.archived,
        })
        .collect();

    let total = response.len();
    Ok(Json(ExperiencesListResponse {
        experiences: response,
        total,
        limit,
        offset: params.offset,
    }))
}

async fn get_experience(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExperienceDetailResponse>> {
    let exp_id = pulsedb::ExperienceId(id);

    let experience = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.get_experience(exp_id)
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??
    .ok_or_else(|| Error::NotFound(format!("Experience {id} not found")))?;

    Ok(Json(ExperienceDetailResponse {
        id: experience.id.to_string(),
        content: experience.content.clone(),
        experience_type: format!("{:?}", experience.experience_type),
        importance: experience.importance,
        confidence: experience.confidence,
        applications: experience.applications,
        domain: experience.domain.clone(),
        related_files: experience.related_files.clone(),
        source_agent: experience.source_agent.0.clone(),
        timestamp_ms: experience.timestamp.0,
    }))
}

async fn list_relations(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<RelationsListResponse>> {
    let collective = pulsedb::CollectiveId(params.collective_id);
    let limit = params.limit.min(1000);

    let relations = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.list_relations(collective, limit, params.offset)
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    let response: Vec<RelationResponse> = relations
        .iter()
        .map(|rel| RelationResponse {
            id: rel.id.to_string(),
            source_id: rel.source_id.to_string(),
            target_id: rel.target_id.to_string(),
            relation_type: format!("{:?}", rel.relation_type),
            strength: rel.strength,
        })
        .collect();

    let total = response.len();
    Ok(Json(RelationsListResponse {
        relations: response,
        total,
    }))
}

async fn list_insights(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<InsightsListResponse>> {
    let collective = pulsedb::CollectiveId(params.collective_id);
    let limit = params.limit.min(1000);

    let insights = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.list_insights(collective, limit, params.offset)
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    let response: Vec<InsightResponse> = insights
        .iter()
        .map(|ins| InsightResponse {
            id: ins.id.to_string(),
            content: ins.content.clone(),
            insight_type: format!("{:?}", ins.insight_type),
            confidence: ins.confidence,
            source_count: ins.source_experience_ids.len(),
            domain: ins.domain.clone(),
        })
        .collect();

    let total = response.len();
    Ok(Json(InsightsListResponse {
        insights: response,
        total,
    }))
}

async fn list_collectives(
    State(state): State<AppState>,
) -> Result<Json<CollectivesListResponse>> {
    let collectives = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || reader.list_collectives()
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    let response: Vec<CollectiveResponse> = collectives
        .iter()
        .map(|c| CollectiveResponse {
            id: c.id.to_string(),
            name: c.name.clone(),
        })
        .collect();

    Ok(Json(CollectivesListResponse {
        collectives: response,
    }))
}

async fn get_stats(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<StatsResponse>> {
    let collective = pulsedb::CollectiveId(params.collective_id);

    let (exp_count, rel_count, ins_count, dim) = tokio::task::spawn_blocking({
        let reader = state.substrate.clone();
        move || -> Result<(usize, usize, usize, usize)> {
            // CollectiveStats only has experience_count, so count relations/insights via list
            let stats = reader.get_collective_stats(collective)?;
            let rels = reader.list_relations(collective, 1, 0)?;
            let ins = reader.list_insights(collective, 1, 0)?;
            let dim = reader.embedding_dimension();
            Ok((stats.experience_count as usize, rels.len(), ins.len(), dim))
        }
    })
    .await
    .map_err(|e| Error::Substrate(e.to_string()))??;

    Ok(Json(StatsResponse {
        experience_count: exp_count,
        relation_count: rel_count,
        insight_count: ins_count,
        embedding_dimension: dim,
    }))
}
