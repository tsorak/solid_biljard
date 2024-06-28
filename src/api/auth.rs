pub mod email_code;
pub mod password;
use axum::{routing::post, Router};

pub fn auth_router() -> Router<crate::State> {
    Router::new()
        .route("/email_code/new", post(email_code::new_code))
        .route("/email_code/validate", post(email_code::validate_code))
        .route("/login", post(password::login))
        .route("/register", post(password::register))
}
