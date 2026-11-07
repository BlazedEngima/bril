use bril_rs::{Type, ValueOps};

use crate::types::{ConstantKey, ConstantValue, ValueNumber};

fn is_bool_constant(value: &Option<ConstantKey>, expected: bool) -> bool {
    matches!(
        value,
        Some(ConstantKey {
            ty: Type::Bool,
            value: ConstantValue::Bool(actual),
        }) if *actual == expected
    )
}

pub(crate) fn fold_arithmetic_operation(
    op: &ValueOps,
    operands: &[Option<ConstantKey>],
) -> Option<ConstantKey> {
    let constants: Option<Vec<&ConstantKey>> = operands.iter().map(Option::as_ref).collect();

    let constants = constants?;
    let (ty, value) = match (op, constants.as_slice()) {
        // Integer arithmetic
        (ValueOps::Add, [a, b])
        | (ValueOps::Sub, [a, b])
        | (ValueOps::Mul, [a, b])
        | (ValueOps::Div, [a, b]) => {
            if a.ty != Type::Int || b.ty != Type::Int {
                return None;
            }

            let (ConstantValue::Int(lhs), ConstantValue::Int(rhs)) = (&a.value, &b.value) else {
                return None;
            };

            let result = match op {
                ValueOps::Add => lhs.wrapping_add(*rhs),
                ValueOps::Sub => lhs.wrapping_sub(*rhs),
                ValueOps::Mul => lhs.wrapping_mul(*rhs),
                ValueOps::Div => {
                    if *rhs == 0 {
                        return None;
                    }

                    lhs.wrapping_div(*rhs)
                }
                _ => unreachable!(),
            };

            (Type::Int, ConstantValue::Int(result))
        }

        // Equality
        (ValueOps::Eq, [a, b]) => {
            if a.ty != b.ty {
                return None;
            }

            let result = match (&a.value, &b.value) {
                (ConstantValue::Int(lhs), ConstantValue::Int(rhs)) => lhs == rhs,

                (ConstantValue::Bool(lhs), ConstantValue::Bool(rhs)) => lhs == rhs,

                _ => return None,
            };

            (Type::Bool, ConstantValue::Bool(result))
        }

        // Integer comparisons
        (ValueOps::Lt | ValueOps::Gt | ValueOps::Le | ValueOps::Ge, [a, b]) => {
            if a.ty != Type::Int || b.ty != Type::Int {
                return None;
            }

            let (ConstantValue::Int(lhs), ConstantValue::Int(rhs)) = (&a.value, &b.value) else {
                return None;
            };

            let result = match op {
                ValueOps::Lt => lhs < rhs,
                ValueOps::Gt => lhs > rhs,
                ValueOps::Le => lhs <= rhs,
                ValueOps::Ge => lhs >= rhs,
                _ => unreachable!(),
            };

            (Type::Bool, ConstantValue::Bool(result))
        }

        // TODO: Add support for float operations and pointer operations
        _ => return None,
    };

    Some(ConstantKey { ty, value })
}

pub(crate) fn fold_logical_operation(
    op: &ValueOps,
    operands: &[Option<ConstantKey>],
) -> Option<ConstantKey> {
    let value = match (op, operands) {
        (ValueOps::Not, [Some(a)]) => {
            if a.ty != Type::Bool {
                return None;
            }

            let ConstantValue::Bool(value) = &a.value else {
                return None;
            };

            !value
        }

        (ValueOps::And, [lhs, rhs]) => {
            if is_bool_constant(lhs, false) || is_bool_constant(rhs, false) {
                false
            } else if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                if lhs.ty != Type::Bool || rhs.ty != Type::Bool {
                    return None;
                }

                let (ConstantValue::Bool(lhs), ConstantValue::Bool(rhs)) = (&lhs.value, &rhs.value)
                else {
                    return None;
                };

                *lhs && *rhs
            } else {
                return None;
            }
        }

        (ValueOps::Or, [lhs, rhs]) => {
            if is_bool_constant(lhs, true) || is_bool_constant(rhs, true) {
                true
            } else if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                if lhs.ty != Type::Bool || rhs.ty != Type::Bool {
                    return None;
                }

                let (ConstantValue::Bool(lhs), ConstantValue::Bool(rhs)) = (&lhs.value, &rhs.value)
                else {
                    return None;
                };

                *lhs || *rhs
            } else {
                return None;
            }
        }

        _ => return None,
    };

    Some(ConstantKey {
        ty: Type::Bool,
        value: ConstantValue::Bool(value),
    })
}

// Fold operations for variables with equal VNs
pub(crate) fn fold_self_comparison(op: &ValueOps, arg_vns: &[ValueNumber]) -> Option<ConstantKey> {
    let [lhs, rhs] = arg_vns else {
        return None;
    };

    if lhs != rhs {
        return None;
    }

    let value = match op {
        ValueOps::Eq => true,
        ValueOps::Lt | ValueOps::Gt => false,
        ValueOps::Le | ValueOps::Ge => true,
        _ => return None,
    };

    Some(ConstantKey {
        ty: Type::Bool,
        value: ConstantValue::Bool(value),
    })
}
