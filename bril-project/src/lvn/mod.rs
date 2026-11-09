mod core;
mod fold;

pub use core::LocalValueNumbering;
use fold::{fold_const_arithmetic_operation, fold_const_logical_operation, fold_self_comparison};
