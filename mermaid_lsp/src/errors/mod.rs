mod error_codes;
pub use error_codes::*;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Response {
    #[serde(rename = "result")]
    Result(serde_json::Value),
    #[serde(rename = "error")]
    Error(ResponseError),
}

#[derive(Debug, Serialize)]
pub struct ResponseError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}
