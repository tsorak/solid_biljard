use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

use crate::api;

pub fn router() -> Router<crate::State> {
    Router::new()
        .fallback_service(get_service(ServeDir::new("client/dist")))
        .nest("/api", api::api_router())
}
