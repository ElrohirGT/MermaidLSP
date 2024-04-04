use serde::Deserialize;

use crate::{jsonrpc::TextDocumentItem, ServerState};

/// Params supplied to the `textDocument/didOpen` method.
#[derive(Debug, Deserialize)]
pub struct DidOpenTextDocumentParams {
    /// The document that was opened
    #[serde(rename = "textDocument")]
    text_document: TextDocumentItem,
}

pub enum DidOpenTextDocumentErrors {}

pub fn did_open_notification(
    state: ServerState,
    params: Option<serde_json::Value>,
) -> Result<ServerState, DidOpenTextDocumentErrors> {
    Ok(state)
}
