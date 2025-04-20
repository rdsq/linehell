use super::func::BuiltinFunc;
use super::data_types::DataTypes;
use std::io::{self, Write};
use super::var_state::VarState;

fn run_block(state: &mut VarState, block: &Vec<super::parse_line::ParsedLine>, name: &str) -> DataTypes {
    let mut internal_state = super::state::State::new(state);
    for (i, line) in block.iter().enumerate() {
        internal_state.interpret_line(&line);
        if let DataTypes::Err(err) = &internal_state.var_state.context_var {
            return DataTypes::Err(format!("[in block \"{}\":{}] {}", name, i+1, err));
        }
    }
    internal_state.var_state.context_var.clone()
}

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
                print!("{}", state.get_var(val).to_string());
            }
            println!();
            DataTypes::None
        }))),
    );
    // arithmetic operations
    functions.insert(
        "math".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let mut sp = args.split_whitespace();
            if let (Some(arg1), Some(operator), Some(arg2)) = (sp.next(), sp.next(), sp.next()) {
                if let (DataTypes::Number(num1), DataTypes::Number(num2)) = (state.get_var(arg1), state.get_var(arg2)) {
                    return match operator {
                        "+" => DataTypes::Number(num1 + num2),
                        "-" => DataTypes::Number(num1 - num2),
                        "*" => DataTypes::Number(num1 * num2),
                        "/" => DataTypes::Number(num1 / num2),
                        _ => DataTypes::Err("Unknown operator".to_string()),
                    };
                } else {
                    DataTypes::Err("Cannot do math with non numbers".to_string())
                }
            } else {
                DataTypes::Err("Incorrect `math <num1> <operator> <num2>` format".to_string())
            }
        }))),
    );
    // convert to number
    functions.insert(
        "number".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            if let DataTypes::String(arg) = state.get_var(&args) {
                return match arg.parse::<f32>() {
                    Ok(num) => DataTypes::Number(num),
                    Err(err) => DataTypes::Err(err.to_string()),
                }
            } else {
                return DataTypes::Err("Not a string".to_string());
            }
        }))),
    );
    // convert to boolean
    functions.insert(
        "bool".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            if let DataTypes::String(arg) = state.get_var(&args) {
                return match &arg as &str {
                    "true" => DataTypes::Bool(true),
                    "false" => DataTypes::Bool(false),
                    _ => DataTypes::Err("Unknown boolean value".to_string()),
                }
            } else {
                return DataTypes::Err("Not a string".to_string());
            }
        }))),
    );
    // output without new line
    functions.insert(
        "printnonl".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let mut is_first = true;
            for val in args.split_whitespace() {
                if !is_first {
                    print!(" ");
                } else {
                    is_first = false;
                }
                print!("{}", state.get_var(val).to_string());
            }
            io::stdout().flush().unwrap();
            DataTypes::None
        }))),
    );
    // stdin input
    functions.insert(
        "input".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|_args, _state| {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.pop(); // remove the \n
            DataTypes::String(input)
        }))),
    );
    // run a block
    functions.insert(
        "run".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            return if let DataTypes::Block(lines) = state.get_var(&args) {
                run_block(state, &lines, &args)
            } else {
                DataTypes::Err("Not a block".to_string())
            }
        }))),
    );
    // if else block
    functions.insert(
        "if-else".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let mut sp = args.split_whitespace();
            return if let (Some(condition), Some(if_block_var), Some(else_block_var)) = (sp.next(), sp.next(), sp.next()) {
                return if let (DataTypes::Block(if_block), DataTypes::Block(else_block)) = (state.get_var(if_block_var), state.get_var(else_block_var)) {
                    return if state.get_var(condition).to_bool() {
                        run_block(state, &if_block, if_block_var)
                    } else {
                        run_block(state, &else_block, else_block_var)
                    }
                } else {
                    DataTypes::Err("if and else statements must be blocks".to_string())
                }
            } else {
                DataTypes::Err("Incorrect `if-else <condition> <if> <else>` format".to_string())
            }
        }))),
    );
    // check if items are equal
    functions.insert(
        "equal".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let sp: Vec<&str> = args.split_whitespace().collect();
            for i in &sp {
                if state.get_var(&i) != state.get_var(&sp[0]) {
                    return DataTypes::Bool(false);
                }
            }
            DataTypes::Bool(true)
        }))),
    );
    // loop
    functions.insert(
        "while".to_string(),
        Box::new(BuiltinFunc::new(Box::new(|args, state| {
            let mut sp = args.split_whitespace();
            return if let (Some(condition), Some(block_var)) = (sp.next(), sp.next()) {
                return if let DataTypes::Block(block) = state.get_var(block_var) {
                    while state.get_var(condition).to_bool() {
                        run_block(state, &block, block_var);
                    }
                    DataTypes::None
                } else {
                    DataTypes::Err("the loop must be a block".to_string())
                }
            } else {
                DataTypes::Err("Incorrect `while <condition> <block>` format".to_string())
            }
        }))),
    );
}
