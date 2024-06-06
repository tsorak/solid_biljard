use axum::{
    extract::{Path, State},
    http::StatusCode as SC,
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
    let year = path.year;
    let month = path.month;

    match state.db.booked_days.get_during(year, month).await {
        Ok(data) => (SC::OK, serde_json::to_string(&data).unwrap()),
        Err(err) => {
            dbg!(err);

            (SC::INTERNAL_SERVER_ERROR, "Server error".to_string())
        }
    }
}
