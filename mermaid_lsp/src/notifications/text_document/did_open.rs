use log::{debug, error, info};
use serde::Deserialize;

use crate::{jsonrpc::TextDocumentItem, MermaidAST, ServerState};

/// Params supplied to the `textDocument/didOpen` method.
#[derive(Debug, Deserialize)]
pub struct DidOpenTextDocumentParams {
    /// The document that was opened
    #[serde(rename = "textDocument")]
    text_document: TextDocumentItem,
}

#[derive(Debug)]
pub enum DidOpenTextDocumentErrors {
    NoParamsSupplied,
    InvalidParams(serde_json::Error),
    FileAlreadyOpened,
}

/// The document open notification is sent from the client to the server to signal newly opened text documents.
/// The document’s content is now managed by the client and the server must not try to read the document’s content using the document’s Uri.
/// Open in this sense means it is managed by the client. It doesn’t necessarily mean that its content is presented in an editor.
/// An open notification must not be sent more than once without a corresponding close notification send before.
/// This means open and close notification must be balanced and the max open count for a particular textDocument is one.
/// Note that a server’s ability to fulfill requests is independent of whether a text document is open or closed.
///
/// The DidOpenTextDocumentParams contain the language id the document is associated with.
/// If the language id of a document changes, the client needs to send a textDocument/didClose to the server followed by a textDocument/didOpen
/// with the new language id if the server handles the new language id as well.
pub fn did_open_notification(
    state: &mut ServerState,
    params: Option<serde_json::Value>,
) -> Result<(), DidOpenTextDocumentErrors> {
    info!("Parsing params to didOpen notification...");

    let params = params.ok_or(DidOpenTextDocumentErrors::NoParamsSupplied)?;
    let DidOpenTextDocumentParams {
        text_document: TextDocumentItem { uri, text, .. },
    } = serde_json::from_value(params).map_err(DidOpenTextDocumentErrors::InvalidParams)?;
    info!("Params parsed!");

    info!("Trying to open file {}", uri);
    match state.documents.entry(uri) {
        std::collections::hash_map::Entry::Occupied(_) => {
            error!("The file is already opened!");
            Err(DidOpenTextDocumentErrors::FileAlreadyOpened)
        }
        std::collections::hash_map::Entry::Vacant(e) => {
            info!("Generating abstract tree for file...");
            let ast = generate_mermaid_ast(text);

            debug!("Updating state...");
            e.insert(ast);
            Ok(())
        }
    }
}

fn generate_mermaid_ast(file_contents: String) -> MermaidAST {
    MermaidAST::default()
}
