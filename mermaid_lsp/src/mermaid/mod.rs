mod diagram_body;
mod diagram_header;

use std::default;

use serde::Deserialize;

use self::{
    diagram_body::parse_diagram,
    diagram_header::{parse_header, MermaidDiagramHeader},
};

#[derive(Debug, Default)]
pub enum MermaidDiagramTypes {
    /// Type that represents when the server couldn't figure out the diagram type
    #[default]
    Unknown,
    Flowchart,
    Sequence,
    Class,
    EntityRelationship,
    UserJourney,
    Gantt,
    Pie,
    Quadrant,
    Requirement,
    Gitgraph,
    C4,
    Mindmap,
    Timeline,
    Zenumi,
    Sankey,
    XY,
    Block,
}

#[derive(Debug, Default)]
pub enum MermaidDiagramDirection {
    #[default]
    TopToBottom,
    BottomToTop,
    RightToLeft,
    LeftToRight,
}

#[derive(Debug, Default)]
pub struct DiagramAST {
    d_type: MermaidDiagramTypes,
}

/// Represents the state of a mermaid file.
#[derive(Debug, Default)]
pub struct MermaidAST {
    header: Option<MermaidDiagramHeader>,
    diagram: DiagramAST,
}

impl MermaidAST {
    pub fn from_content(content: String) -> Self {
        let (rest, header) = match parse_header(&content).ok() {
            Some((r, h)) => (r, Some(h)),
            None => (content, None),
        };
        let diagram = parse_diagram(&rest);

        MermaidAST { header, diagram }
    }
}
