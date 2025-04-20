use super::parse_line::parse_line;
use super::state::State;

pub fn repl() {
    let mut state = State::new();
    loop {
        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            break;
        }
        input.pop();
        if let Some(parsed) = parse_line(&input) {
            let _ = state.interpret_line(&parsed);
            println!("\x1b[2m{}\x1b[0m", state.var_state.context_var.to_string());
        }
    }
}
