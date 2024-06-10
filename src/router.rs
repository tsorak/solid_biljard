use axum::{
    response::{IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

use crate::api;

pub fn router() -> Router<crate::State> {
    index_router()
        .fallback_service(get_service(ServeDir::new("client/dist")))
        .nest("/api", api::api_router())
}

fn index_router() -> Router<crate::State> {
    Router::new()
        .route("/", get(index))
        .route("/book", get(index))
        .route("/auth/email_code", get(index))
}

async fn index() -> Response {
    use crate::ext::res;
    use axum::response::Html;
    use tokio::fs;

    let html = match fs::read_to_string("./client/dist/index.html").await {
        Ok(v) => v,
        Err(_) => return res::text(500, "Server failed to get index.html"),
    };

    Html(html).into_response()
}
