use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get_service,
    Router,
};
use tower_http::services::ServeDir;

use crate::api;

pub fn router() -> Router<crate::State> {
    Router::new()
        .fallback_service(get_service(ServeDir::new("client/dist")))
        .layer(middleware::from_fn(rebuild_client))
        .nest("/api", api::api_router())
}

async fn rebuild_client(_headers: HeaderMap, request: Request, next: Next) -> impl IntoResponse {
    let p = request.uri().path();

    if p == "/" {
        println!("Building client...");
        crate::build::build_client().await;
    }

    next.run(request).await
}
