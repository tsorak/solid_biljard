use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn book_router() -> Router<crate::State> {
    Router::new().route("/:year/:month", get(booked_days))
}

#[derive(Debug, Deserialize)]
struct BookedDaysParams {
    year: u16,
    month: u8,
}

async fn booked_days(
    state: State<crate::State>,
    path: Path<BookedDaysParams>,
) -> impl IntoResponse {
    dbg!(&path);

    (
        StatusCode::OK,
        serde_json::json!([{ "day": 5_u8, "bookedBy": "foo" }]).to_string(),
    )
}
