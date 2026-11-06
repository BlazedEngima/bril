use crate::types::lvn_types::ValueNumber;
use bril_rs::{Type, ValueOps};

/// The tuple struct for expressions in the LVN table
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Expression {
    pub op: ValueOps,
    pub args: Vec<ValueNumber>,
    pub op_type: Type,
}
