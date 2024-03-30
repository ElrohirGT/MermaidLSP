use log::{debug, info};
use serde::Deserialize;

#[derive(Debug)]
pub enum InitializeRequestErrors {
    ParamsParsingError(serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct InitializeRequestParams {
    client_info: Option<ClientInfo>,
    client_capabilities: ClientCapabilities,
}

#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    name: String,
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
    applyEdit: Option<bool>,
}

pub fn initialize_request(params: serde_json::Value) -> Result<(), InitializeRequestErrors> {
    let params: InitializeRequestParams =
        serde_json::from_value(params).map_err(InitializeRequestErrors::ParamsParsingError)?;
    info!(
        "Successfully parsed params for `initialize` request! {:?}",
        params
    );
    Ok(())
}
