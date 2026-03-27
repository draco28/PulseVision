pub mod events;
pub mod substrate;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(events::router())
        .merge(substrate::router())
}
