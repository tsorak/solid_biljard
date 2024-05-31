use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

pub fn router() -> Router<crate::State> {
    Router::new()
        .fallback_service(get_service(ServeDir::new("client/dist")))
        .layer(middleware::from_fn(rebuild_client))
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

async fn rebuild_client(_headers: HeaderMap, request: Request, next: Next) -> impl IntoResponse {
    let p = request.uri().path();

    if p == "/" {
        println!("Building client...");
        crate::build::build_client().await;
    }

    next.run(request).await
}
