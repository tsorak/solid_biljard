use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Router,
};
use serde::Deserialize;

use crate::ext::res;

pub fn book_router() -> Router<crate::State> {
    Router::new().route("/:year/:month", get(booked_days))
}

#[derive(Debug, Deserialize)]
struct BookedDaysParams {
    year: u16,
    month: u8,
}

async fn booked_days(state: State<crate::State>, path: Path<BookedDaysParams>) -> Response {
    let year = path.year;
    let month = path.month;

    match state.db.booked_days.get_during(year, month).await {
        Ok(data) => res::json(200, data),
        Err(err) => {
            dbg!(err);
            res::str(500, "Server error")
        }
    }
}
