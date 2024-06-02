use axum::{routing::get, Router};

mod motd;
mod ws;

pub fn api_router() -> Router<crate::State> {
    Router::new()
        .route("/", get(status::status))
        .route("/motd", get(motd::motd))
        .route("/ws", get(ws::ws))
}

pub(super) mod status {
    use axum::{http::StatusCode, response::IntoResponse};

    pub async fn status() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
}
