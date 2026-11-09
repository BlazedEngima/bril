use bril_rs::ValueOps;

use crate::types::{ConstantKey, ConstantType, ConstantValue, LVNTable, ValueNumber};

fn get_operands<T: ConstantType>(lhs: &ConstantKey, rhs: &ConstantKey) -> Option<(T, T)> {
    let lhs = T::extract(&lhs.value)?;
    let rhs = T::extract(&rhs.value)?;

    Some((lhs, rhs))
}

pub fn fold_const_arithmetic_operation(
    op: ValueOps,
    operands: &[ValueNumber],
    table: &LVNTable,
) -> Option<ConstantKey> {
    let [_, _] = operands else {
        return None;
    };

    let lhs = table.get_constant_from_vn(operands.first()?)?;
    let rhs = table.get_constant_from_vn(operands.get(1)?)?;

    let (lhs, rhs) = get_operands::<i64>(lhs, rhs)?;
    let result = match op {
        ValueOps::Add | ValueOps::Sub | ValueOps::Mul | ValueOps::Div => {
            let val = match op {
                ValueOps::Add => lhs.wrapping_add(rhs),
                ValueOps::Sub => lhs.wrapping_sub(rhs),
                ValueOps::Mul => lhs.wrapping_mul(rhs),
                ValueOps::Div => {
                    if rhs == 0 {
                        return None;
                    }
                    lhs.wrapping_div(rhs)
                }
                _ => unreachable!(),
            };

            ConstantValue::Int(val)
        }

        ValueOps::Eq | ValueOps::Lt | ValueOps::Gt | ValueOps::Le | ValueOps::Ge => {
            let val = match op {
                ValueOps::Eq => lhs == rhs,
                ValueOps::Lt => lhs < rhs,
                ValueOps::Gt => lhs > rhs,
                ValueOps::Le => lhs <= rhs,
                ValueOps::Ge => lhs >= rhs,
                _ => unreachable!(),
            };

            ConstantValue::Bool(val)
        }

        _ => return None,
    };

    Some(ConstantKey { value: result })
}

pub fn fold_const_logical_operation(
    op: ValueOps,
    operands: &[ValueNumber],
    table: &LVNTable,
) -> Option<ConstantKey> {
    if op == ValueOps::Not {
        let [operand] = operands else {
            return None;
        };

        let val = table
            .get_constant_from_vn(operand)
            .and_then(|constant| bool::extract(&constant.value))?;

        return Some(ConstantKey {
            value: ConstantValue::Bool(!val),
        });
    }

    let [lhs, rhs] = operands else {
        return None;
    };

    let lhs = table
        .get_constant_from_vn(lhs)
        .and_then(|constant| bool::extract(&constant.value));

    let rhs = table
        .get_constant_from_vn(rhs)
        .and_then(|constant| bool::extract(&constant.value));

    let result = match op {
        ValueOps::And => match (lhs, rhs) {
            (Some(false), _) | (_, Some(false)) => false,
            (Some(true), Some(true)) => true,
            _ => return None,
        },

        ValueOps::Or => match (lhs, rhs) {
            (Some(true), _) | (_, Some(true)) => true,
            (Some(false), Some(false)) => false,
            _ => return None,
        },

        _ => return None,
    };

    Some(ConstantKey {
        value: ConstantValue::Bool(result),
    })
}

pub fn fold_self_comparison(op: ValueOps, operands: &[ValueNumber]) -> Option<ConstantKey> {
    let [lhs, rhs] = operands else {
        return None;
    };

    if lhs != rhs {
        return None;
    }

    let value = match op {
        ValueOps::Lt | ValueOps::Gt => false,
        ValueOps::Eq | ValueOps::Le | ValueOps::Ge => true,
        _ => return None,
    };

    Some(ConstantKey {
        value: ConstantValue::Bool(value),
    })
}
