use std::collections::HashMap;
use super::data_types::DataTypes;
use super::func::LangFunc;

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
    pub fn get_va(&self, name: &str) -> DataTypes {
        if name == "that" {
            return self.context_var.clone();
        }
        return match self.variables.get(name) {
            Some(value) => value.clone(),
            None => DataTypes::None,
        }
    }
}
