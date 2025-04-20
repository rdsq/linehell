use std::collections::HashMap;
use super::data_types::DataTypes;
use super::func::LangFunc;
use super::parse_line::ParsedLine;
use super::var_state::VarState;

pub struct State {
    var_state: VarState,
    functions: HashMap<String, Box<dyn LangFunc>>,
}

impl State {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        super::builtins::init_builtins(&mut functions);
        Self {
            functions,
            var_state: VarState::new(),
        }
    }
    pub fn run_func(&mut self, name: &str, args: String) -> DataTypes {
        return match self.functions.get(name) {
            Some(func) => func.call(args, &mut self.var_state),
            None => DataTypes::Err("Unknown function".to_string()),
        }
    }
    pub fn interpret_line(&mut self, line: &ParsedLine) -> Result<(), String> {
        self.var_state.context_var = self.run_func(&line.func, line.args.to_string());
        if let DataTypes::Err(err) = &self.var_state.context_var {
            return Err(err.to_string());
        }
        Ok(())
    }
}
