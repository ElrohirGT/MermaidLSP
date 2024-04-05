mod diagram_header;

use std::default;

use serde::Deserialize;

use self::diagram_header::{parse_header, MermaidDiagramHeader};

#[derive(Debug, Default)]
pub enum MermaidDiagramTypes {
    #[default]
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

/// Represents the state of a mermaid file.
#[derive(Debug, Default)]
pub struct MermaidAST {
    header: Option<MermaidDiagramHeader>,
    diagram_type: MermaidDiagramTypes,
    diagram_direction: MermaidDiagramDirection,
}

impl MermaidAST {
    pub fn from_content(content: String) -> Self {
        let header = parse_header(&content).ok();

        MermaidAST {
            header,
            diagram_type: MermaidDiagramTypes::default(),
            diagram_direction: MermaidDiagramDirection::default(),
        }
    }
}
