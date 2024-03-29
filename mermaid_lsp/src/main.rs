use log::error;
use log::info;
use mermaid_lsp::LSPMessages;
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
                "Message received!\nLength: {}\nContent: {:?}",
                message.len(),
                String::from_utf8_lossy(&message)
            );
        }
        Err(e) => error!("An error ocurred while recieving message! {:?}", e),
    })
}
