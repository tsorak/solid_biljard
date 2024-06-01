use axum::{routing::get, Router};

mod motd;

pub fn api_router() -> Router<crate::State> {
    Router::new()
        .route("/", get(status::status))
        .route("/motd", get(motd::motd))
}

pub(super) mod status {
    use axum::{http::StatusCode, response::IntoResponse};

    pub async fn status() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
}
