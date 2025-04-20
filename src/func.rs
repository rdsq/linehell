use super::data_types::DataTypes;
use super::state::State;

pub trait LangFunc {
    fn call(&self, args: String, state: &mut State) -> DataTypes;
}

pub struct BuiltinFunc {
    func: Box<dyn Fn(String, &mut State) -> DataTypes>,
}

impl LangFunc for BuiltinFunc {
    fn call(&self, args: String, state: &mut State) -> DataTypes {
        (self.func)(args, state)
    }
}

impl BuiltinFunc {
    pub fn new(func: Box<dyn Fn(String, &mut State) -> DataTypes>) -> Self {
        Self { func }
    }
}
