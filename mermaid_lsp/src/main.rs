use log::debug;
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
use mermaid_lsp::notifications::text_document::did_open_notification;
use mermaid_lsp::requests::initialize_request;
use mermaid_lsp::requests::shutdown_request;
use mermaid_lsp::ServerState;
use simplelog::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::ControlFlow;

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
    let mut messages = LSPMessages::new(reader);

    messages.try_fold(
        ServerState::default(),
        |state, message| match handle_message(state, message) {
            ServerAction::Ignore(new_state) => ControlFlow::Continue(new_state),
            ServerAction::Exit => ControlFlow::Break(()),
            ServerAction::Respond(new_state, response) => {
                let response = match encode_message(response) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("The response couldn't be serialized into a string! {:?}", e);
                        return ControlFlow::Break(());
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

                info!("Response sent!");

                ControlFlow::Continue(new_state)
            }
        },
    );

    info!("The server is exiting...");
}

/// Enum that represents all actions the server can take when it recieves a `ClientMessage`
enum ServerAction {
    Respond(ServerState, ServerResponse),
    Ignore(ServerState),
    Exit,
}

/// Handles a possible incoming `ClientMessage`.
fn handle_message(
    mut state: ServerState,
    message: Result<ClientMessage, ParseJsonRPCMessageErrors>,
) -> ServerAction {
    match message {
        Ok(message) => {
            info!("Message received! {:?}", message);

            match (state.initialized, message) {
                (false, ClientMessage::Request { id, method, params })
                    if method == *"initialize" =>
                {
                    let response = initialize_request(id, params);
                    let initialized = matches!(response, ServerResponse::Result { .. });
                    ServerAction::Respond(
                        ServerState {
                            initialized,
                            ..state
                        },
                        response,
                    )
                }

                (false, _) => {
                    warn!(
                        "Message recieved and valid but an initialize request has not been sent!"
                    );
                    let response = ServerResponse::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::ServerNotInitialized,
                            "The server needs to be initialized first with a `initialize` request!"
                                .into(),
                        ),
                    );

                    ServerAction::Respond(state, response)
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

                    ServerAction::Respond(state, response)
                }

                (true, ClientMessage::Request { id, method, params }) => {
                    // Handle requests other than initialize...
                    match method.as_str() {
                        "shutdown" => {
                            info!("Shutting down the server with id: {}", id);
                            ServerAction::Respond(state, shutdown_request(id))
                        }
                        _ => {
                            warn!("Unimplemented request received!");
                            let response = ServerResponse::new_error(
                                None,
                                ResponseError::new(
                                    ErrorCodes::InvalidRequest,
                                    "This method is not implemented yet!".into(),
                                ),
                            );

                            ServerAction::Respond(state, response)
                        }
                    }
                }

                (true, ClientMessage::Notification { method, params }) => {
                    // Handle notifications...
                    match method.as_str() {
                        "exit" => ServerAction::Exit,
                        "textDocument/didOpen" => match did_open_notification(&mut state, params) {
                            Ok(_) => ServerAction::Ignore(state),
                            Err(e) => {
                                error!("An error ocurred while opening file: {:?}", e);
                                ServerAction::Ignore(state)
                            }
                        },
                        _ => {
                            warn!("Unimplemented notification received! Ignoring...");
                            ServerAction::Ignore(state)
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

            ServerAction::Respond(state, response)
        }
    }
}
