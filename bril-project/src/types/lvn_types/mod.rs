mod constant;
mod expression;
mod table;

pub type ValueNumber = u32;

pub use constant::{ConstantKey, ConstantType, ConstantValue};
pub use expression::Expression;
pub use table::LVNTable;
