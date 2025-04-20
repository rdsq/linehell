use super::func::BuiltinFunc;
use super::data_types::DataTypes;

pub fn init_builtins(functions: &mut std::collections::HashMap<String, Box<dyn super::func::LangFunc>>) {
    functions.insert(
        "set".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|argv, state| {
            DataTypes::None
        }))),
    );
}
