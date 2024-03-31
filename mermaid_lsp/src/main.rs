use log::error;
use log::info;
use log::warn;
use mermaid_lsp::errors::ErrorCodes;
use mermaid_lsp::errors::Response;
use mermaid_lsp::errors::ResponseError;
use mermaid_lsp::requests::initialize_request;
use mermaid_lsp::LSPMessages;
use mermaid_lsp::LspId;
use mermaid_lsp::ParseJsonRPCMessageErrors;
use serde::Deserialize;
use simplelog::*;
use std::fs::File;
use std::io;
use std::io::Write;

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
    messages.for_each(|message| {
        let response = handle_message(&mut initialized, message);
        let response = match serde_json::to_string(&response) {
            Ok(v) => v,
            Err(e) => {
                error!("The response couldn't be serialized into a string! {:?}", e);
                return;
            }
        };

        let mut handle = io::stdout().lock();

        info!("Sending response: {}", response);
        if let Err(e) = writeln!(handle, "{}", response) {
            error!("An error occurred while writing to STDOUT {:?}", e);
        }

        info!("Response sent!")
    })
}

fn handle_message(
    initialized: &mut bool,
    message: Result<String, ParseJsonRPCMessageErrors>,
) -> Response {
    match message {
        Ok(message) => {
            info!(
                "Message received! With length: {} - Body: {}",
                message.len(),
                message
            );

            let body: LspMessageBody = match serde_json::from_str(&message) {
                Ok(v) => v,
                Err(e) => {
                    error!("Message body couldn't be parsed from JSON! {:?}", e);
                    return Response::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::ParseError,
                            "The body coudln't be parsed into an object!".into(),
                        ),
                    );
                }
            };
            info!("Message parsed from JSON: {:?}", body);

            match (*initialized, body.method.as_str()) {
                (false, "initialize") => {
                    let response = initialize_request(body.id, body.params);
                    *initialized = matches!(response, Response::Result { .. });
                    response
                }

                (false, _) => {
                    warn!(
                        "Message recieved and valid but an initialize request has not been sent!"
                    );
                    Response::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::ServerNotInitialized,
                            "The server need to be initialized first with a `initialize` request!"
                                .into(),
                        ),
                    )
                }

                (true, "initialize") => {
                    warn!("Initialize request received but server is already initialized!");
                    Response::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::InvalidRequest,
                            "The server is already initialized!".into(),
                        ),
                    )
                }

                _ => {
                    warn!("Unimplemented method received!");
                    Response::new_error(
                        None,
                        ResponseError::new(
                            ErrorCodes::InvalidRequest,
                            "This method is not implemented yet!".into(),
                        ),
                    )
                }
            }
        }
        Err(e) => {
            error!("An error ocurred while recieving message! {:?}", e);
            Response::new_error(
                None,
                ResponseError::new(
                    ErrorCodes::InvalidRequest,
                    "An error occurred while trying to recieve a message!".into(),
                ),
            )
        }
    }
}

#[derive(Debug, Deserialize)]
struct LspMessageBody {
    id: LspId,
    method: String,
    params: serde_json::Value,
}
