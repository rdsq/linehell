pub struct ParsedLine {
    pub func: String,
    pub args: String,
}

fn parse_functional_line(line: &str) -> ParsedLine {
    if let Some((func, args)) = line.split_once(' ') {
        ParsedLine {
            func: func.to_string(),
            args: args.to_string(),
        }
    } else {
        ParsedLine {
            func: line.to_string(),
            args: String::from(""),
        }
    }
}

pub fn parse_line(line: &str) -> Option<ParsedLine> {
    let trimmed = line.trim();
    if trimmed == "" {
        return None;
    } else {
        return Some(parse_functional_line(trimmed));
    }
}
