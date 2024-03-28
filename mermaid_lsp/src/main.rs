use log::info;
use simplelog::*;
use std::fs::File;

fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("/home/elrohirgt/Documents/Development/MermaidLSP/mermaid_lsp.log").unwrap(),
    )
    .unwrap();

    info!("Logging setup correctly!");
}
