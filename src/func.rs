use super::data_types::DataTypes;
use super::var_state::VarState;

pub trait LangFunc {
    fn call(&self, args: String, state: &mut VarState) -> DataTypes;
}

pub struct BuiltinFunc {
    func: Box<dyn Fn(String, &mut VarState) -> DataTypes>,
}

impl LangFunc for BuiltinFunc {
    fn call(&self, args: String, state: &mut VarState) -> DataTypes {
        (self.func)(args, state)
    }
}

impl BuiltinFunc {
    pub fn new(func: Box<dyn Fn(String, &mut VarState) -> DataTypes>) -> Self {
        Self { func }
    }
}
