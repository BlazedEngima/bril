mod core;
mod fold;

pub use core::LocalValueNumbering;
pub(crate) use fold::{fold_arithmetic_operation, fold_logical_operation, fold_self_comparison};
