use super::types::Type;

pub enum Purity {
    Impure,
    Effected,
    Undetermined,
    Pure,
}

pub struct Argument {
    name: String,
    r#type: Type,
}

pub struct Function {
    name: String,
    purity: Purity,
    r#type: Type,
}

pub struct FunctionCall {
    function: Function,
    arguments: Vec<Argument>,
}
