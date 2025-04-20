#[derive(Clone)]
pub enum DataTypes {
    String(String),
    Int(i32),
    Bool(bool),
    None,
    Err(String),
}
