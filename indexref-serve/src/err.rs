use std::fmt::Display;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;

pub trait ErrorStatusCode {
    fn status_code(&self) -> StatusCode;
}

pub trait ToJsonResultResponse {
    fn to_json_result_response(self) -> Response;
}

impl ErrorStatusCode for eyre::Report {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl<T, E> ToJsonResultResponse for Result<T, E>
where
    T: Serialize,
    E: ErrorStatusCode + Display,
{
    fn to_json_result_response(self) -> Response {
        match self {
            Ok(value) => Json(json!({
                "status": "ok",
                "value": value
            }))
            .into_response(),
            Err(err) => (
                err.status_code(),
                Json(json!({
                    "status": "error",
                    "error": err.to_string()
                })),
            )
                .into_response(),
        }
    }
}
