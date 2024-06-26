use axum::{routing::get, Router};

mod auth;
mod book;
mod ws;

pub mod types;

pub fn api_router() -> Router<crate::State> {
    Router::new()
        .route("/", get(status::status))
        .route("/ws", get(ws::ws))
        .nest("/auth", auth::auth_router())
        .nest("/book", book::book_router())
}

pub(super) mod status {
    use axum::{http::StatusCode, response::IntoResponse};

    pub async fn status() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
}
