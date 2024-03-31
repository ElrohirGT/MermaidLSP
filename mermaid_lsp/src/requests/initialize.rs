use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorCodes, Response, ResponseError};

#[derive(Debug)]
pub enum InitializeRequestErrors {
    ParamsParsingError(serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct InitializeRequestParams {
    /// Information about the client
    ///
    /// @since 3.15.0
    #[serde(rename = "clientInfo")]
    client_info: Option<AppInfo>,

    /// The capabilities provided by the client (editor or tool)
    capabilities: ClientCapabilities,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    /// The name of the app as defined by the app.
    name: String,

    /// The apps's version as defined by the app..
    version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClientCapabilities {
    workspace: Option<WorkspaceCapabilities>,
}

/// Workspace specific client capabilities.
#[derive(Debug, Deserialize)]
pub struct WorkspaceCapabilities {
    /// The client supports applying batch edits to the workspace by supporting the request
    /// 'workspace/applyEdit'
    #[serde(rename = "applyEdit")]
    apply_edit: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct InitializeResult {
    /// The capabilities the language server provides
    capabilities: ServerCapabilities,

    /// Information about the server
    #[serde(rename = "serverInfo")]
    server_info: AppInfo,
}

#[derive(Debug, Serialize)]
pub struct ServerCapabilities {}

pub fn initialize_request(params: serde_json::Value) -> Response {
    let params: InitializeRequestParams = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => {
            error!(
                "An error occurred while trying to parse initialize request params {:?}",
                e
            );
            return Response::Error(ResponseError::new(
                ErrorCodes::InvalidParams,
                "Invalid params supplied to initialize request!".into(),
            ));
        }
    };
    info!(
        "Successfully parsed params for `initialize` request! {:?}",
        params
    );

    Response::Result(serde_json::Value::Null)
}
