use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Motd {
    motd: String,
}

pub async fn motd(State(state): State<crate::State>) -> impl IntoResponse {
    let motd = Motd {
        motd: format!("Message of the day is:\n{}", state.motd),
    };

    Json(motd)
}
