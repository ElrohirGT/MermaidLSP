mod diagram_body;
mod diagram_header;
mod flowchart;

use self::{
    diagram_body::parse_diagram,
    diagram_header::{parse_header, MermaidDiagramHeader},
};

/// The different types of tokens we analyze from a mermaid diagram
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TokenType {
    DiagramDirection,
    Node,
}

/// Represents a token of the mermaid language
#[derive(Debug, PartialEq, Eq)]
pub struct MermaidToken {
    pub content: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum MermaidDiagramTypes {
    /// Type that represents when the server couldn't figure out the diagram type
    #[default]
    Unknown,
    Flowchart,
    Sequence,
    Class,
    State,
    EntityRelationship,
    UserJourney,
    Gantt,
    Pie,
    Quadrant,
    Requirement,
    Gitgraph,
    Mindmap,
    Timeline,
    Zenumi,
}

#[derive(Debug, Default, PartialEq, Eq)]
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
