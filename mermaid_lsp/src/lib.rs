use std::collections::HashMap;

use serde::Deserialize;

pub mod jsonrpc;
pub mod notifications;
pub mod requests;

/// Represents the state of a mermaid file.
#[derive(Debug, Deserialize)]
pub struct MermaidAST {}

impl Default for MermaidAST {
    fn default() -> Self {
        Self {}
    }
}

/// Represents the whole state of the server
#[derive(Debug)]
pub struct ServerState {
    /// All documents that have been opened and the LSP recognizes.
    /// Consists of a key that is the URI of the file and a value
    /// that is the AST from the parsed file.
    pub documents: HashMap<String, MermaidAST>,

    /// Flag that indicates whether or not the server has been initialized.
    pub initialized: bool,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            documents: Default::default(),
            initialized: false,
        }
    }
}
