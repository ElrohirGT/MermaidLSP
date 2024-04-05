use std::collections::HashMap;

use super::{MermaidDiagramDirection, MermaidToken, TokenType};

/// Parses all the data relevant to a flow chart
fn parse_flowchart(content: &str) -> HashMap<TokenType, Vec<MermaidToken>> {
    let mut tokens = HashMap::default();

    let direction = parse_direction(content);
    tokens
        .entry(TokenType::DiagramDirection)
        .or_insert(vec![direction]);

    tokens
}

/// Parses a flowchart direction
fn parse_direction(content: &str) -> MermaidToken {
    let line = content.lines().next().unwrap_or("flowchart TD");
    let direction = line.split(' ').last().unwrap_or("TD");

    MermaidToken {
        content: direction.to_string(),
        line: 1,
        column: 11,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SUCCESS_DIAGRAM: &str = r#"flowchart LR
    A[Start] --> B{Is it?}
    B -- Yes --> C[OK]
    C --> D[Rethink]
    D --> B
    B -- No ----> E[End]
"#;

    #[test]
    fn parse_direction_success() {
        let expected = MermaidToken {
            content: "LR".to_string(),
            line: 1,
            column: 11,
        };
        let actual = parse_direction(SUCCESS_DIAGRAM);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_flowchart_success() {
        let expected_nodes = vec![
            MermaidToken {
                content: "A".to_string(),
                line: 4,
                column: 1,
            },
            MermaidToken {
                content: "B".to_string(),
                line: 2,
                column: 17,
            },
            MermaidToken {
                content: "C".to_string(),
                line: 3,
                column: 17,
            },
            MermaidToken {
                content: "D".to_string(),
                line: 4,
                column: 10,
            },
            MermaidToken {
                content: "E".to_string(),
                line: 6,
                column: 18,
            },
        ];
        let tokens = parse_flowchart(SUCCESS_DIAGRAM);

        let nodes = &tokens[&TokenType::Node];

        println!("EXPECTED: {:?}", expected_nodes);
        println!("ACTUAL: {:?}", nodes);

        let all_are_in_expected = nodes.iter().all(|n| expected_nodes.contains(n));
        assert!(all_are_in_expected);
    }
}
