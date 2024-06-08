use axum::{routing::get, Router};

mod auth;
mod book;
mod motd;
mod ws;

pub use auth::email_code;

pub fn api_router() -> Router<crate::State> {
    Router::new()
        .route("/", get(status::status))
        .route("/motd", get(motd::motd))
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
