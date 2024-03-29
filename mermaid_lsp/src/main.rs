use log::error;
use log::info;
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
    messages.for_each(|message| match message {
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
                    return;
                }
            };

            info!("Message parsed from JSON: {:?}", body);
        }
        Err(e) => error!("An error ocurred while recieving message! {:?}", e),
    })
}

#[derive(Debug, Deserialize)]
struct LspMessageBody {
    id: u32,
    method: String,
}
