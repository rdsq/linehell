use super::data_types::DataTypes;
use super::state::State;

pub trait LangFunc {
    fn call(&self, argv: Vec<String>, state: &mut State) -> DataTypes;
}

pub struct BuiltinFunc {
    func: Box<dyn Fn(Vec<String>, &mut State) -> DataTypes>,
}

impl LangFunc for BuiltinFunc {
    fn call(&self, argv: Vec<String>, state: &mut State) -> DataTypes {
        (self.func)(argv, state)
    }
}

impl BuiltinFunc {
    pub fn new(func: Box<dyn Fn(Vec<String>, &mut State) -> DataTypes>) -> Self {
        Self { func }
    }
}
