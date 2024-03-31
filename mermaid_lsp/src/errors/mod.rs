mod error_codes;
pub use error_codes::*;

use serde::Serialize;

use crate::LspId;

pub const JSON_RPC_VERSION: &str = "2.0";

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response {
    Result {
        jsonrpc: String,
        id: Option<LspId>,
        result: serde_json::Value,
    },
    Error {
        jsonrpc: String,
        id: Option<LspId>,
        error: ResponseError,
    },
}

impl Response {
    pub fn new_result(id: Option<LspId>, result: serde_json::Value) -> Self {
        Response::Result {
            jsonrpc: JSON_RPC_VERSION.into(),
            id,
            result,
        }
    }
    pub fn new_error(id: Option<LspId>, error: ResponseError) -> Self {
        Response::Error {
            jsonrpc: JSON_RPC_VERSION.into(),
            id,
            error,
        }
    }
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
