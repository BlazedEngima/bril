use crate::types::{
    BasicBlock, ConstantKey, ConstantValue, Expression, LVNTable, ProgramCFG, ValueNumber,
};
use bril_rs::{ConstOps, Instruction, Literal, Type, ValueOps};

pub struct LocalValueNumbering;

impl LocalValueNumbering {
    fn is_commutative(op: &ValueOps) -> bool {
        matches!(
            op,
            ValueOps::Add | ValueOps::Mul | ValueOps::Eq | ValueOps::And | ValueOps::Or
        )
    }

    fn canonicalize_args(op: &ValueOps, args: &mut [ValueNumber]) {
        if Self::is_commutative(op) {
            args.sort_unstable();
        }
    }

    fn is_lvn_eligible(op: &ValueOps) -> bool {
        matches!(
            op,
            ValueOps::Add
                | ValueOps::Sub
                | ValueOps::Mul
                | ValueOps::Div
                | ValueOps::Eq
                | ValueOps::Lt
                | ValueOps::Gt
                | ValueOps::Le
                | ValueOps::Ge
                | ValueOps::Not
                | ValueOps::And
                | ValueOps::Or
        )
    }

    fn literal_to_constant(literal: &Literal) -> ConstantValue {
        match literal {
            Literal::Int(i) => ConstantValue::Int(*i),
            Literal::Bool(b) => ConstantValue::Bool(*b),
            Literal::Float(f) => ConstantValue::Float(f.to_bits()),
            Literal::Char(c) => ConstantValue::Char(*c),
        }
    }

    fn constant_to_literal(value: &ConstantValue) -> Literal {
        match value {
            ConstantValue::Int(value) => Literal::Int(*value),
            ConstantValue::Bool(bool) => Literal::Bool(*bool),
            ConstantValue::Float(bits) => Literal::Float(f64::from_bits(*bits)),
            ConstantValue::Char(char) => Literal::Char(*char),
        }
    }

    fn fold_operation(op: &ValueOps, operands: &[ConstantKey]) -> Option<ConstantKey> {
        let (ty, value) = match (op, operands) {
            (ValueOps::Not, [a]) => {
                if a.ty != Type::Bool {
                    return None;
                }

                let ConstantValue::Bool(value) = &a.value else {
                    return None;
                };

                (Type::Bool, ConstantValue::Bool(!value))
            }

            // Integer arithmetic
            (ValueOps::Add, [a, b])
            | (ValueOps::Sub, [a, b])
            | (ValueOps::Mul, [a, b])
            | (ValueOps::Div, [a, b]) => {
                if a.ty != Type::Int || b.ty != Type::Int {
                    return None;
                }

                let (ConstantValue::Int(lhs), ConstantValue::Int(rhs)) = (&a.value, &b.value)
                else {
                    return None;
                };

                let result = match op {
                    ValueOps::Add => lhs.checked_add(*rhs)?,
                    ValueOps::Sub => lhs.checked_sub(*rhs)?,
                    ValueOps::Mul => lhs.checked_mul(*rhs)?,
                    ValueOps::Div => lhs.checked_div(*rhs)?,
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

                let (ConstantValue::Int(lhs), ConstantValue::Int(rhs)) = (&a.value, &b.value)
                else {
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

            // Boolean operations
            (ValueOps::And | ValueOps::Or, [a, b]) => {
                if a.ty != Type::Bool || b.ty != Type::Bool {
                    return None;
                }

                let (ConstantValue::Bool(lhs), ConstantValue::Bool(rhs)) = (&a.value, &b.value)
                else {
                    return None;
                };

                let result = match op {
                    ValueOps::And => *lhs && *rhs,
                    ValueOps::Or => *lhs || *rhs,
                    _ => unreachable!(),
                };

                (Type::Bool, ConstantValue::Bool(result))
            }

            // TODO: Add support for float operations and pointer operations

            // Unsupported operation or incorrect operand count.
            _ => return None,
        };

        Some(ConstantKey { ty, value })
    }

    fn proccess_instruction(instr: &mut Instruction, table: &mut LVNTable) {
        match instr {
            Instruction::Constant {
                dest,
                const_type,
                value,
                ..
            } => {
                let key = ConstantKey {
                    ty: const_type.clone(),
                    value: Self::literal_to_constant(value),
                };

                let vn = match table.get_vn_from_constant(&key) {
                    Some(vn) => vn,
                    None => {
                        let vn = table.fresh_value_number();

                        table.insert_constant(key, vn);

                        vn
                    }
                };

                // Assign destination to this value number
                table.assign_vn_to_variable(dest.clone(), vn);
            }

            Instruction::Value {
                op,
                dest,
                args,
                funcs,
                labels,
                op_type,
                pos,
            } => {
                let arg_vns: Vec<ValueNumber> = args
                    .iter()
                    .map(|arg| table.get_or_create_vn_from_variable(arg))
                    .collect();

                // Special handling for id: Replace with Const
                if *op == ValueOps::Id {
                    let source_vn = arg_vns[0];

                    // Propagate constants when the source is known to be constant.
                    if let Some(constant) = table.get_constant_from_vn(&source_vn).cloned() {
                        let literal = Self::constant_to_literal(&constant.value);

                        table.assign_vn_to_variable(dest.clone(), source_vn);

                        *instr = Instruction::Constant {
                            dest: dest.clone(),
                            op: ConstOps::Const,
                            const_type: constant.ty,
                            value: literal,
                            pos: pos.clone(),
                        };

                        return;
                    }

                    table.assign_vn_to_variable(dest.clone(), source_vn);
                    return;
                }

                if !Self::is_lvn_eligible(op) {
                    let vn = table.fresh_value_number();
                    table.assign_vn_to_variable(dest.clone(), vn);
                    return;
                }

                // Try constant folding
                let operand_constants: Option<Vec<ConstantKey>> = arg_vns
                    .iter()
                    .map(|vn| table.get_constant_from_vn(vn).cloned())
                    .collect();

                if let Some(operand_constants) = operand_constants
                    && let Some(folded_constant) = Self::fold_operation(op, &operand_constants)
                {
                    let vn = table.get_or_create_vn_from_constant(folded_constant.clone());

                    let replacement = Instruction::Constant {
                        dest: dest.clone(),
                        op: ConstOps::Const,
                        const_type: folded_constant.ty.clone(),
                        value: Self::constant_to_literal(&folded_constant.value),
                        pos: pos.clone(),
                    };

                    table.assign_vn_to_variable(dest.clone(), vn);

                    *instr = replacement;
                    return;
                }

                let mut expression_arg_vns = arg_vns;

                Self::canonicalize_args(op, &mut expression_arg_vns);

                // Common Subexoression Elimination
                let expression = Expression {
                    op: *op,
                    args: expression_arg_vns,
                    op_type: op_type.clone(),
                };

                // Check whether this expression has already been evaluated.
                if let Some(existing_vn) = table.get_vn_from_expression(&expression) {
                    // Only replace the computation if another variable currently
                    // holds the value. Expression entries can outlive their variables.
                    if let Some(var) = table.get_variable_from_vn_except(existing_vn, dest) {
                        *op = ValueOps::Id;

                        args.clear();
                        args.push(var.to_owned());

                        funcs.clear();
                        labels.clear();
                    }

                    table.assign_vn_to_variable(dest.clone(), existing_vn);
                } else {
                    // First occurrence of this expression.
                    let vn = table.fresh_value_number();

                    table.insert_expression(expression, vn);
                    table.assign_vn_to_variable(dest.clone(), vn);
                }
            }

            _ => {}
        }
    }

    fn process_block(block: &mut BasicBlock) {
        let mut table = LVNTable::new();

        for instr in block.get_instrs_mut() {
            Self::proccess_instruction(instr, &mut table);
        }
    }

    pub fn run(program: &mut ProgramCFG) {
        for function in program.get_functions_mut() {
            for basic_block in function.get_blocks_mut() {
                Self::process_block(basic_block);
            }
        }
    }
}
