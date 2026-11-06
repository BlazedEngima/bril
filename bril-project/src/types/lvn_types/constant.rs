use bril_rs::Type;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ConstantValue {
    Int(i64),
    Bool(bool),
    Float(u64),
    Char(char),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConstantKey {
    pub ty: Type,
    pub value: ConstantValue,
}

impl ConstantKey {
    pub fn new(ty: Type, value: ConstantValue) -> Self {
        Self { ty, value }
    }
}
