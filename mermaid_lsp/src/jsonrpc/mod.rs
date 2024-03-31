mod decoder;
mod encoder;
mod error_codes;

use std::fmt::Display;

pub use decoder::*;
pub use encoder::*;
pub use error_codes::*;
use serde::{Deserialize, Serialize};

/// The JSON RPC version currently used
pub const JSON_RPC_VERSION: &str = "2.0";

/// A request/response id, it can be either a string or an integer
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LspId {
    String(String),
    Integer(i32),
}

// Display implementation for LspId, mainly for logs.
impl Display for LspId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LspId::String(s) => f.write_str(s),
            LspId::Integer(i) => f.write_fmt(format_args!("{}", i)),
        }
    }
}

/// Represents the message an LSP client sends to this server
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ClientMessage {
    Request {
        id: LspId,
        method: String,
        params: Option<serde_json::Value>,
    },
    Notification {
        method: String,
        params: Option<serde_json::Value>,
    },
}

/// Represents the response the server sends to a `ClientMessage`
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ServerResponse {
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

impl ServerResponse {
    /// Creates a new `ServerResponse` that is successful and with the given result.
    pub fn new_result(id: Option<LspId>, result: serde_json::Value) -> Self {
        ServerResponse::Result {
            jsonrpc: JSON_RPC_VERSION.into(),
            id,
            result,
        }
    }

    /// Creates a new `ServerResponse` that failed with the given `ResponseError`.
    pub fn new_error(id: Option<LspId>, error: ResponseError) -> Self {
        ServerResponse::Error {
            jsonrpc: JSON_RPC_VERSION.into(),
            id,
            error,
        }
    }
}

/// Represents a response that signals an error
#[derive(Debug, Serialize)]
pub struct ResponseError {
    /// The code of the errror according to `ErrorCodes`
    code: i32,

    /// A custom message for the error
    message: String,

    /// Optional data to supply context for the given error
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
