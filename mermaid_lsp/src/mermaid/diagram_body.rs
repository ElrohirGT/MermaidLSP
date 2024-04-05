use super::{DiagramAST, MermaidDiagramTypes};

#[derive(Debug)]
pub enum ParseDiagramBodyErrors {}

/// Parses an entire diagram content into a struct
pub fn parse_diagram(content: &str) -> DiagramAST {
    let d_type = parse_diagram_type(content);
    DiagramAST { d_type }
}

/// Attempts to parse a diagram type from a line
fn parse_diagram_type(type_line: &str) -> MermaidDiagramTypes {
    let type_string = type_line.split_whitespace().next();
    match type_string {
        Some("flowchart") => MermaidDiagramTypes::Flowchart,
        _ => MermaidDiagramTypes::Unknown,
    }
}
