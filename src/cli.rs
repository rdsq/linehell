use super::parse_module::parse_module;
use super::state::State;
use std::process::exit;

pub fn cli() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| {
            eprintln!("Provide the file path");
            exit(1);
        });
    if &path == "--help" || &path == "-h" {
        println!("Linelang: a super mimimalistic programming language");
        return;
    }
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| {
            eprintln!("Failed to read the file: {}", err);
            exit(1);
        });
    let parsed = parse_module(&content);
    let mut state = State::new();
    let resulting = state.interpret(parsed);
    if let Err((message, line)) = resulting {
        eprintln!("Error (:{}): {}", line + 1, message);
        eprintln!("-----");
        eprintln!("{}", content.split('\n').nth(line).unwrap());
        exit(1);
    }
}
