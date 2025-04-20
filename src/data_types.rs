#[derive(Clone, Debug)]
pub enum DataTypes {
    String(String),
    Number(f32),
    Bool(bool),
    None,
    Err(String),
    Block(Vec<super::parse_line::ParsedLine>),
}

impl DataTypes {
    pub fn to_string(&self) -> String {
        return match self {
            Self::String(v) => v.clone(),
            Self::Number(v) => v.to_string(),
            Self::Bool(v)   => v.to_string(),
            Self::None      => "none".to_string(),
            Self::Err(v)    => format!("Error: {}", v),
            Self::Block(v)  => format!("{{\n{}\n}}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")),
        }
    }
}
