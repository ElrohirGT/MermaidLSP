use crate::jsonrpc::ServerResponse;

#[derive(Debug)]
pub enum EncodeErrors {
    MessageSerializationError(serde_json::Error),
}

/// Encodes a `ServerResponse` according to the LSP spec.
pub fn encode_message(msg: ServerResponse) -> Result<String, EncodeErrors> {
    let body = serde_json::to_string(&msg).map_err(EncodeErrors::MessageSerializationError)?;
    let content_length = body.len();

    Ok(format!(
        "Content-Length: {}\r\n\r\n{}",
        content_length, body
    ))
}
