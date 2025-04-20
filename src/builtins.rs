use super::func::BuiltinFunc;
use super::data_types::DataTypes;

pub fn init_builtins(functions: &mut std::collections::HashMap<String, Box<dyn super::func::LangFunc>>) {
    // set variable
    functions.insert(
        "set".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let mut a = args.split_whitespace();
            if let (Some(k), Some(v)) = (a.next(), a.next()) {
                state.set_var(k.to_string(), state.get_var(v));
            } else {
                return DataTypes::Err("Incorrect `set <key> <value>` format".to_string());
            }
            DataTypes::None
        }))),
    );
    // create string
    functions.insert(
        "str".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, _state| {
            DataTypes::String(args) // that simple
        }))),
    );
    // create number
    functions.insert(
        "number".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, _state| {
            return match args.parse::<f32>() {
                Ok(num) => DataTypes::Number(num),
                Err(err) => DataTypes::Err(err.to_string()),
            }
        }))),
    );
    // create boolean
    functions.insert(
        "bool".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, _state| {
            return match &args as &str {
                "true" => DataTypes::Bool(true),
                "false" => DataTypes::Bool(false),
                _ => DataTypes::Err("Unknown boolean value".to_string()),
            }
        }))),
    );
    // output
    functions.insert(
        "print".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let mut is_first = true;
            for val in args.split_whitespace() {
                if !is_first {
                    print!(" ");
                } else {
                    is_first = false;
                }
                print!("{:?}", state.get_var(val)); // temporary
            }
            println!();
            DataTypes::None
        }))),
    );
}
