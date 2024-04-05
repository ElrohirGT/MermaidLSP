/// The header of a Mermaid diagram
#[derive(Debug)]
pub struct MermaidDiagramHeader {
    /// The title of a Mermaid diagram
    title: String,
}

/// Enum that contains errors when parsing a diagram header
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
        lines
            .fold(String::new(), |acc, e| acc + "\n" + e),
        MermaidDiagramHeader { title },
    ))
}

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
