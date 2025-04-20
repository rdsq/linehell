use super::data_types::DataTypes;
use std::collections::HashMap;

pub struct VarState {
    variables: HashMap<String, DataTypes>,
    pub context_var: DataTypes,
}

impl VarState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            context_var: DataTypes::None,
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
}
