use log::error;
use log::info;
use log::warn;
use mermaid_lsp::errors::ErrorCodes;
use mermaid_lsp::errors::Response;
use mermaid_lsp::errors::ResponseError;
use mermaid_lsp::requests::initialize_request;
use mermaid_lsp::LSPMessages;
use serde::Deserialize;
use simplelog::*;
use std::fs::File;

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
        .map(|message| match message {
            Ok(message) => {
                info!(
                    "Message received!\nLength: {}\nContent: {}",
                    message.len(),
                    message
                );

                let body: LspMessageBody = match serde_json::from_str(&message) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("Message body couldn't be parsed from JSON! {:?}", e);
                        return Response::Error(ResponseError::new(
                            ErrorCodes::ParseError,
                            "The body couldn't be parsed into an object!".into(),
                        ));
                    }
                };
                info!("Message parsed from JSON: {:?}", body);

                match (initialized, body.method.as_str()) {
                    (false, "initialize") => {
                        let response = initialize_request(body.params);
                        initialized =  matches!(response, Response::Result(_));
                        response
                    },

                    (false, _) => {
                        warn!("Message recieved and valid but an initialize request has not been sent!");
                        Response::Error(ResponseError::new(ErrorCodes::ServerNotInitialized, "The server need to be initialized first with a `initialize` request!".into()))
                    },

                    (true, "initialize") => {
                        warn!("Initialize request received but server is already initialized!");
                        Response::Error(ResponseError::new(ErrorCodes::InvalidRequest, "The server is already initialized!".into()))
                    },

                    _ => {
                        warn!("Unimplemented method received!");
                        Response::Error(ResponseError::new(
                            ErrorCodes::InvalidRequest,
                            "This method is not implemented yet!".into(),
                        ))
                    }
                }
            }
            Err(e) => {
                error!("An error ocurred while recieving message! {:?}", e);
                Response::Error(ResponseError::new(
                    ErrorCodes::InvalidRequest,
                    "An error occurred while trying to recieve a message!".into(),
                ))
            }
        })
        .for_each(|response| {
            let response = serde_json::to_string(&response)
                .expect("Error response couldn't be converted serialized!");
            println!("{}", response);
        })
}

#[derive(Debug, Deserialize)]
struct LspMessageBody {
    method: String,
    params: serde_json::Value,
}
