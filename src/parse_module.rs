use super::parse_line::{ParsedLine, parse_line};

pub struct ParsedLineMeta {
    line: ParsedLine,
    line_number: usize,
}

pub fn parse_module(content: &str) -> Vec<ParsedLineMeta> {
    let mut lines = Vec::new();
    for (i, line) in content.split('\n').enumerate() {
        if let Some(parsed) = parse_line(line) {
            lines.push(ParsedLineMeta {
                line: parsed,
                line_number: i,
            });
        } // ignore empty lines
    }
    lines
}
