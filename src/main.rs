mod data_types;
mod func;
mod builtins;
mod state;
mod parse_line;
mod parse_module;
mod var_state;
mod file_interpret;
mod repl;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        file_interpret::file_interpret(&path);
    } else {
        println!("Linelang REPL");
        repl::repl();
    }
}
