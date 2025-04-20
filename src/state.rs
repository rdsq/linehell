use std::collections::HashMap;
use super::data_types::DataTypes;
use super::func::LangFunc;
use super::parse_line::ParsedLine;
use super::var_state::VarState;

pub struct State {
    pub var_state: VarState, // because of some mut borrowing issues
    functions: HashMap<String, Box<dyn LangFunc>>,
    pub current_block: Option<Vec<ParsedLine>>,
}

impl State {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        super::builtins::init_builtins(&mut functions);
        Self {
            functions,
            var_state: VarState::new(),
            current_block: None,
        }
    }
    pub fn run_func(&mut self, name: &str, args: String) -> DataTypes {
        return match self.functions.get(name) {
            Some(func) => func.call(args, &mut self.var_state),
            None => DataTypes::Err("Unknown function".to_string()),
        }
    }
    pub fn interpret_line(&mut self, line: &ParsedLine) {
        return if self.current_block.is_none() && line.func == "{" {
            self.current_block = Some(Vec::new());
        } else if self.current_block.is_some() && line.func == "}" {
            self.var_state.context_var = DataTypes::Block(self.current_block.clone().unwrap());
            self.current_block = None;
        } else if let Some(block) = &mut self.current_block {
            block.push(line.clone());
        } else {
            self.var_state.context_var = self.run_func(&line.func, line.args.to_string());
        }
    }
}
