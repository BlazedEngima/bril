use bril_rs::Type;

// TODO: add support for float, pointer, and any types
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ConstantValue {
    Int(i64),
    Bool(bool),
    Float(u64),
    Char(char),
}

pub trait ConstantType: Sized + Copy {
    fn extract(value: &ConstantValue) -> Option<Self>;
}

impl ConstantType for i64 {
    fn extract(value: &ConstantValue) -> Option<Self> {
        match value {
            ConstantValue::Int(value) => Some(*value),
            _ => None,
        }
    }
}

impl ConstantType for bool {
    fn extract(value: &ConstantValue) -> Option<Self> {
        match value {
            ConstantValue::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

impl ConstantType for char {
    fn extract(value: &ConstantValue) -> Option<Self> {
        match value {
            ConstantValue::Char(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConstantKey {
    pub value: ConstantValue,
}

impl ConstantKey {
    #[must_use]
    pub const fn new(value: ConstantValue) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn ty(&self) -> Type {
        match self.value {
            ConstantValue::Int(_) => Type::Int,
            ConstantValue::Bool(_) => Type::Bool,
            ConstantValue::Float(_) => Type::Float,
            ConstantValue::Char(_) => Type::Char,
        }
    }
}
