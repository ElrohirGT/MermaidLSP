/// The header of a Mermaid diagram
#[derive(Debug)]
pub struct MermaidDiagramHeader {
    /// The title of a Mermaid diagram
    title: String,
}

/// Enum that contains errors when parsing a diagram header
#[derive(Debug, PartialEq, Eq)]
pub enum ParseHeaderErrors {
    NotEnoughHeaderLines,
    InvalidTopDelimiterFormat,
    InvalidBottomDelimiterFormat,
    TitleFormatError(ParseTitleErrors),
}

/// Function that attempts to extract a header from a mermaid content file
pub fn parse_header(content: &str) -> Result<(String, MermaidDiagramHeader), ParseHeaderErrors> {
    let mut lines = content.trim_start().lines();

    let header_top_delim = lines
        .next()
        .ok_or(ParseHeaderErrors::NotEnoughHeaderLines)?;

    let title_line = lines
        .next()
        .ok_or(ParseHeaderErrors::NotEnoughHeaderLines)?;

    let header_bottom_delim = lines
        .next()
        .ok_or(ParseHeaderErrors::NotEnoughHeaderLines)?;

    if header_top_delim != "---" {
        Err(ParseHeaderErrors::InvalidTopDelimiterFormat)?
    }
    if header_top_delim != header_bottom_delim {
        Err(ParseHeaderErrors::InvalidBottomDelimiterFormat)?
    }

    let title = parse_title(title_line).map_err(ParseHeaderErrors::TitleFormatError)?;

    Ok((
        lines.fold(String::new(), |acc, e| acc + "\n" + e),
        MermaidDiagramHeader { title },
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseTitleErrors {
    IncorrectTitleFormat,
}

/// Function that attempts to parse a mermaid diagram title from a line.
fn parse_title(title_line: &str) -> Result<String, ParseTitleErrors> {
    if title_line.starts_with("title:") {
        Ok(title_line
            .chars()
            .skip(6)
            .collect::<String>()
            .trim()
            .to_string())
    } else {
        Err(ParseTitleErrors::IncorrectTitleFormat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_title_success() {
        let title = "A normal mermaid title";
        let title_line = format!("title: {}", title);

        let result = parse_title(&title_line).unwrap();

        assert_eq!(result, title);
    }

    #[test]
    fn parse_title_without_space() {
        let title = "A normal mermaid title";
        let title_line = format!("title:{}", title);

        let result = parse_title(&title_line).unwrap();

        assert_eq!(result, title);
    }

    #[test]
    fn parse_title_fail_incorrect_format() {
        let title = "A normal mermaid title";
        let title_line = format!("tite:{}", title);

        let result = parse_title(&title_line).unwrap_err();

        assert_eq!(result, ParseTitleErrors::IncorrectTitleFormat);
    }

    #[test]
    fn parse_header_success() {
        let title = "A normal mermaid title";
        let content = format!(
            r#"---
title:{}
---"#,
            title
        );

        let (rest, header) = parse_header(&content).unwrap();

        assert_eq!(header.title, title);
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_header_fail_top_delimiter() {
        let title = "A normal mermaid title";
        let content = format!(
            r#"--
title:{}
---"#,
            title
        );

        let err = parse_header(&content).unwrap_err();

        assert_eq!(err, ParseHeaderErrors::InvalidTopDelimiterFormat);
    }

    #[test]
    fn parse_header_fail_bottom_delimiter() {
        let title = "A normal mermaid title";
        let content = format!(
            r#"---
title:{}
--"#,
            title
        );

        let err = parse_header(&content).unwrap_err();

        assert_eq!(err, ParseHeaderErrors::InvalidBottomDelimiterFormat);
    }

    #[test]
    fn parse_header_fail_not_enough_lines() {
        let content = "---\n---";

        let err = parse_header(content).unwrap_err();

        assert_eq!(err, ParseHeaderErrors::NotEnoughHeaderLines);
    }
}
