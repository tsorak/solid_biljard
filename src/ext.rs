pub mod res {
    use axum::{
        body::Body,
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };

    use serde::Serialize;

    pub fn json<T>(code: u16, value: T) -> Response
    where
        T: Serialize,
    {
        let mut res = Json(value).into_response();
        *res.status_mut() = StatusCode::from_u16(code).unwrap();
        res
    }

    pub fn str(code: u16, value: &str) -> Response {
        let mut res = Body::from(value.to_string()).into_response();
        *res.status_mut() = StatusCode::from_u16(code).unwrap();
        res
    }
}
