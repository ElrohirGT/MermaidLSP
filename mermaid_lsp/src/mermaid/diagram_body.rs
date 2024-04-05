use super::{DiagramAST, MermaidDiagramTypes};

#[derive(Debug)]
pub enum ParseDiagramBodyErrors {}

/// Parses an entire diagram content into a struct
pub fn parse_diagram(content: &str) -> DiagramAST {
    let d_type = parse_diagram_type(content);
    let data = match d_type {
        MermaidDiagramTypes::Unknown => DiagramAST::default(),
        MermaidDiagramTypes::Flowchart => todo!(),
        _ => todo!(),
    };
    DiagramAST { d_type }
}

/// Attempts to parse a diagram type from a line
fn parse_diagram_type(type_line: &str) -> MermaidDiagramTypes {
    let type_string = type_line.split_whitespace().next();
    match type_string {
        Some("flowchart") => MermaidDiagramTypes::Flowchart,
        Some("sequenceDiagram") => MermaidDiagramTypes::Sequence,
        Some("classDiagram") => MermaidDiagramTypes::Class,
        Some("stateDiagram" | "stateDiagram-v2") => MermaidDiagramTypes::State,
        Some("erDiagram") => MermaidDiagramTypes::EntityRelationship,
        Some("journey") => MermaidDiagramTypes::UserJourney,
        Some("gantt") => MermaidDiagramTypes::Gantt,
        Some("pie") => MermaidDiagramTypes::Pie,
        Some("quadrantChart") => MermaidDiagramTypes::Quadrant,
        Some("requirementDiagram") => MermaidDiagramTypes::Requirement,
        Some("gitGraph") => MermaidDiagramTypes::Gitgraph,
        Some("mindmap") => MermaidDiagramTypes::Mindmap,
        Some("timeline") => MermaidDiagramTypes::Timeline,
        Some("zenuml") => MermaidDiagramTypes::Zenumi,
        _ => MermaidDiagramTypes::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_diagram_success() {
        let diagram = r#"
flowchart TD
    A[Start] --> B{Is it?}
    B -- Yes --> C[OK]
    C --> D[Rethink]
    D --> B
    B -- No ----> E[End]
"#;

        let result = parse_diagram(diagram);
        assert_eq!(result.d_type, MermaidDiagramTypes::Flowchart)
    }

    #[test]
    fn parse_diagram_type_success() {
        let type_lines = [
            "flowchart TD\n",
            "sequenceDiagram\n",
            "classDiagram\n",
            "stateDiagram-v2\n",
            "stateDiagram\n",
            "erDiagram\n",
            "journey\n",
            "gantt\n",
            "pie\n",
            "quadrantChart\n",
            "requirementDiagram\n",
            "gitGraph\n",
            "mindmap\n",
            "timeline\n",
            "zenuml\n",
        ];

        let expected_results = [
            MermaidDiagramTypes::Flowchart,
            MermaidDiagramTypes::Sequence,
            MermaidDiagramTypes::Class,
            MermaidDiagramTypes::State,
            MermaidDiagramTypes::State,
            MermaidDiagramTypes::EntityRelationship,
            MermaidDiagramTypes::UserJourney,
            MermaidDiagramTypes::Gantt,
            MermaidDiagramTypes::Pie,
            MermaidDiagramTypes::Quadrant,
            MermaidDiagramTypes::Requirement,
            MermaidDiagramTypes::Gitgraph,
            MermaidDiagramTypes::Mindmap,
            MermaidDiagramTypes::Timeline,
            MermaidDiagramTypes::Zenumi,
        ];

        type_lines
            .iter()
            .map(|&line| (line, parse_diagram_type(line)))
            .zip(expected_results)
            .for_each(|((input, actual), expected)| {
                assert_eq!(
                    actual, expected,
                    "Input supplied to parse_diagram_type: {}",
                    input
                )
            });
    }
}
