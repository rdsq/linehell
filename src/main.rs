mod data_types;
mod func;
mod builtins;
mod state;
mod parse_line;
mod parse_module;
mod var_state;
mod cli;

fn main() {
    cli::cli();
}
