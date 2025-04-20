use std::collections::HashMap;
use super::data_types::DataTypes;
use super::func::LangFunc;
use super::parse_module::ParsedLineMeta;

pub struct State {
    variables: HashMap<String, DataTypes>,
    context_var: DataTypes,
    functions: HashMap<String, Box<dyn LangFunc>>,
}

impl State {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        super::builtins::init_builtins(&mut functions);
        Self {
            variables: HashMap::new(),
            context_var: DataTypes::None,
            functions,
        }
    }
    pub fn get_var(&self, name: &str) -> DataTypes {
        if name == "that" {
            return self.context_var.clone();
        }
        return match self.variables.get(name) {
            Some(value) => value.clone(),
            None => DataTypes::None,
        }
    }
    pub fn set_var(&mut self, key: String, value: DataTypes) {
        self.variables.insert(key, value);
    }
    pub fn run_func(&mut self, name: &str, args: String) -> DataTypes {
        if let Some(func) = self.functions.get(name) {
            return func.call(args, &mut self);
        } else {
            return DataTypes::Err("Unknown function".to_string());
        }
    }
    pub fn interpret(&mut self, parsed: Vec<ParsedLineMeta>) -> Result<(), (String, usize)> {
        for line in parsed {
            self.context_var = self.run_func(&line.line.func, line.line.args);
            if let DataTypes::Err(err) = &self.context_var {
                return Err((err.to_string(), line.line_number));
            }
        }
        Ok(())
    }
}
