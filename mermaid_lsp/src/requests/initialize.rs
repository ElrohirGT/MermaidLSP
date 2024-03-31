use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorCodes, Response, ResponseError},
    LspId,
};

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
    /// Workspace specific client capabilities.
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
pub struct ServerCapabilities {
    /// Defines how text documents are synced. Is either a detailed structure
    /// defining each notification or for backwards compatibility the
    /// TextDocumentSyncKind number. If omitted it defaults to
    /// `TextDocumentSyncKind.None`.
    #[serde(rename = "textDocumentSync")]
    text_document_sync: u8,
}

/// Defines how the host (editor) should sync document changes to the language
/// server.
#[derive(Debug, Serialize)]
pub enum TextDocumentSyncKind {
    /// Documents should not be synced at all.
    None = 0,

    /// Documents are synced by always sending the full content
    /// of the document.
    Full = 1,

    /// Documents are synced by sending the full content on open.
    /// After that only incremental updates to the document are
    /// sent.
    Incremental = 2,
}

pub fn initialize_request(id: LspId, params: serde_json::Value) -> Response {
    let params: InitializeRequestParams = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => {
            error!(
                "An error occurred while trying to parse initialize request params {:?}",
                e
            );
            return Response::new_error(
                Some(id),
                ResponseError::new(
                    ErrorCodes::InvalidParams,
                    "Invalid params supplied to initialize request!".into(),
                ),
            );
        }
    };
    info!(
        "Successfully parsed params for `initialize` request! {:?}",
        params
    );

    debug!("Generating response...");
    let server_result = InitializeResult {
        server_info: AppInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        },
        capabilities: ServerCapabilities {
            text_document_sync: TextDocumentSyncKind::Full as u8,
        },
    };

    debug!("Response generated {:?}", server_result);
    Response::new_result(
        Some(id),
        serde_json::to_value(server_result)
            .expect("Server message couldn't be serialized into a value!"),
    )
}
