use super::parse_module::parse_module;
use super::state::State;
use std::process::exit;
use super::data_types::DataTypes;

pub fn file_interpret(path: &str) {
    if path == "--help" || path == "-h" {
        println!("Linelang: a super mimimalistic programming language");
        return;
    }
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|err| {
            eprintln!("Failed to read the file: {}", err);
            exit(1);
        });
    let parsed = parse_module(&content);
    let mut state = State::new();
    for line in parsed {
        state.interpret_line(&line.line);
        if let DataTypes::Err(message) = state.var_state.context_var {
            eprintln!("Error (:{}): {}", line.line_number + 1, message);
            eprintln!("-----");
            eprintln!("{} {}", line.line.func, line.line.args);
            exit(1);
        }
    }
}
