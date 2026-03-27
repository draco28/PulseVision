pub mod substrate;
pub mod projections;
pub mod attractors;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(substrate::router())
        .merge(projections::router())
        .merge(attractors::router())
}
