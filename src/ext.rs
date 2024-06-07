pub mod res {
    use axum::{
        body::Body,
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };

    use serde_json::Value;

    pub fn json(code: u16, value: Value) -> Response {
        let mut res = Json::from(value.to_string()).into_response();
        *res.status_mut() = StatusCode::from_u16(code).unwrap();
        res
    }

    pub fn str(code: u16, value: &str) -> Response {
        let mut res = Body::from(value.to_string()).into_response();
        *res.status_mut() = StatusCode::from_u16(code).unwrap();
        res
    }
}
