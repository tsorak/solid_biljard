use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

pub fn router() -> Router<crate::State> {
    Router::new()
        .fallback_service(get_service(ServeDir::new("client/dist")))
        .route("/api", get(status))
        .route("/motd", get(motd))
}

async fn status() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn motd(State(state): State<crate::State>) -> impl IntoResponse {
    let body = format!("Message of the day is:\n{}", state.motd);

    (StatusCode::OK, body)
}
