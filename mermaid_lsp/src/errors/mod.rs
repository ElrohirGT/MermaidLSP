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

impl ResponseError {
    /// Creates a new ResponseError struct
    pub fn new(code: ErrorCodes, message: String) -> Self {
        ResponseError {
            code: code as i32,
            message,
            data: None,
        }
    }

    /// Adds the specified data to the created struct
    pub fn with_data(mut self, data: Option<serde_json::Value>) -> Self {
        self.data = data;
        self
    }
}
