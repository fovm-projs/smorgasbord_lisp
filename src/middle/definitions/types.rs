pub enum Type {
    Integer { unsigned: bool, size: u8, },
    Float { size: u8 },
    r#String,
}
