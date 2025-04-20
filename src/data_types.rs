#[derive(Clone, Debug)]
pub enum DataTypes {
    String(String),
    Number(f32),
    Bool(bool),
    None,
    Err(String),
}
