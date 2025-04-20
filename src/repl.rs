use super::parse_line::parse_line;
use super::state::State;
use super::var_state::VarState;

pub fn repl() {
    let mut var_state = VarState::new();
    let mut state = State::new(&mut var_state);
    loop {
        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            break;
        }
        input.pop();
        if let Some(parsed) = parse_line(&input) {
            state.interpret_line(&parsed);
            if state.current_block.is_none() {
                println!("\x1b[2m{}\x1b[0m", state.var_state.context_var.to_string());
            }
        }
    }
}
