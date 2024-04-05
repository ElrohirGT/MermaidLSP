use std::collections::HashMap;

use mermaid::MermaidAST;

pub mod jsonrpc;
pub mod mermaid;
pub mod notifications;
pub mod requests;

/// Represents the whole state of the server
#[derive(Debug, Default)]
pub struct ServerState {
    /// All documents that have been opened and the LSP recognizes.
    /// Consists of a key that is the URI of the file and a value
    /// that is the AST from the parsed file.
    pub documents: HashMap<String, MermaidAST>,

    /// Flag that indicates whether or not the server has been initialized.
    pub initialized: bool,
}
