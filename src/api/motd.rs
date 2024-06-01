use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn motd(State(state): State<crate::State>) -> impl IntoResponse {
    let body = format!("Message of the day is:\n{}", state.motd);

    (StatusCode::OK, body)
}
