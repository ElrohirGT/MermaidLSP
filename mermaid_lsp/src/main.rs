use log::error;
use log::info;
use log::warn;
use mermaid_lsp::jsonrpc::encode_message;
use mermaid_lsp::jsonrpc::ClientMessage;
use mermaid_lsp::jsonrpc::ErrorCodes;
use mermaid_lsp::jsonrpc::LSPMessages;
use mermaid_lsp::jsonrpc::ParseJsonRPCMessageErrors;
use mermaid_lsp::jsonrpc::ResponseError;
use mermaid_lsp::jsonrpc::ServerResponse;
use mermaid_lsp::requests::initialize_request;
use simplelog::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("/home/elrohirgt/Documents/Development/MermaidLSP/mermaid_lsp.log").unwrap(),
    )
    .unwrap();

    info!("Logging setup correctly!");

    let stdin = std::io::stdin();
    let reader = std::io::BufReader::new(stdin);
    let messages = LSPMessages::new(reader);
    let mut initialized = false;
    messages
        .filter_map(|message| handle_message(&mut initialized, message))
        .for_each(|response| {
            let response = match encode_message(response) {
                Ok(v) => v,
                Err(e) => {
                    error!("The response couldn't be serialized into a string! {:?}", e);
                    return;
                }
            };

            let mut stdout = io::stdout();

            info!("Sending response: {:?}", response);
            if let Err(e) = stdout.write_all(response.as_bytes()) {
                error!("An error occurred while writing to STDOUT {:?}", e);
            }
            if let Err(e) = stdout.flush() {
                error!("An error occurred while flushing STDOUT {:?}", e);
            }

            info!("Response sent!")
        });
}

/// Handles a possible incoming `ClientMessage`. Returns a `ServerResponse` if the server want's to
/// respond to the incoming `ClientMessage`.
fn handle_message(
    initialized: &mut bool,
    message: Result<ClientMessage, ParseJsonRPCMessageErrors>,
) -> Option<ServerResponse> {
    match message {
        Ok(message) => {
            info!("Message received! {:?}", message);

            match (*initialized, message) {
                (false, ClientMessage::Request { id, method, params })
                    if method == *"initialize" =>
                {
                    let response = initialize_request(id, params);
                    *initialized = matches!(response, ServerResponse::Result { .. });
                    Some(response)
                }

                (false, _) => {
                    warn!(
                        "Message recieved and valid but an initialize request has not been sent!"
                    );
                    let response = ServerResponse::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::ServerNotInitialized,
                            "The server need to be initialized first with a `initialize` request!"
                                .into(),
                        ),
                    );

                    Some(response)
                }

                (true, ClientMessage::Request { method, .. }) if method == *"initialize" => {
                    warn!("Initialize request received but server is already initialized!");
                    let response = ServerResponse::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::InvalidRequest,
                            "The server is already initialized!".into(),
                        ),
                    );

                    Some(response)
                }

                (true, ClientMessage::Request { id, method, params }) => {
                    // Handle requests other than initialize...
                    match method {
                        _ => {
                            warn!("Unimplemented request received!");
                            let response = ServerResponse::new_error(
                                None,
                                ResponseError::new(
                                    ErrorCodes::InvalidRequest,
                                    "This method is not implemented yet!".into(),
                                ),
                            );

                            Some(response)
                        }
                    }
                }

                (true, ClientMessage::Notification { method, params }) => {
                    // Handle notifications...
                    match method {
                        _ => {
                            warn!("Unimplemented notification received! Ignoring...");
                            None
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("An error ocurred while recieving message! {:?}", e);
            let response = ServerResponse::new_error(
                None,
                ResponseError::new(
                    ErrorCodes::InvalidRequest,
                    "An error occurred while trying to recieve a message!".into(),
                ),
            );

            Some(response)
        }
    }
}
