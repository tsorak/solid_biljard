use axum::{extract::State, response::Response, Json};
use serde::Deserialize;

use crate::ext::res;

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

pub async fn login(state: State<crate::State>, body: Json<LoginReq>) -> Response {
    let Json(LoginReq {
        username,
        email,
        password,
    }) = body;

    let identifier = login::get_identification_method(email, username);
    if identifier.is_none() {
        return res::Json::new("You must provide either an email or a username to log in with")
            .status(400);
    }

    let user = state
        .db
        .users
        .get_using_identifier(identifier.unwrap())
        .await;

    if user.is_none() {
        return res::Json::new("No user found with that username/email").status(400);
    }
    let user = user.unwrap();

    if user.is_correct_password(password) {
        res::Json::new("Success").status(200)
    } else {
        res::Json::new("Bad login").status(400)
    }
}

mod login {
    use crate::api::types::login::IdentificationMethod;

    pub fn get_identification_method(
        email: Option<String>,
        username: Option<String>,
    ) -> Option<IdentificationMethod> {
        match (email, username) {
            (None, None) => None,
            (Some(email), _) => Some(IdentificationMethod::Email(email)),
            (_, Some(username)) => Some(IdentificationMethod::Username(username)),
        }
    }
}

pub async fn register() -> Response {
    todo!()
}
