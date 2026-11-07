use bril_rs::{ConstOps, Instruction, Literal, Position, ValueOps};

use crate::{
    lvn::{fold_arithmetic_operation, fold_logical_operation, fold_self_comparison},
    types::{
        BasicBlock, ConstantKey, ConstantValue, Expression, LVNTable, ProgramCFG, ValueNumber,
    },
};

pub struct LocalValueNumbering;

impl LocalValueNumbering {
    fn is_commutative(op: &ValueOps) -> bool {
        matches!(
            op,
            ValueOps::Add | ValueOps::Mul | ValueOps::Eq | ValueOps::And | ValueOps::Or
        )
    }

    // Sort commutative operation arguments
    fn canonicalize_expression_args(op: &ValueOps, args: &mut [ValueNumber]) {
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

    fn literal_to_constant(literal: Literal) -> ConstantValue {
        match literal {
            Literal::Int(i) => ConstantValue::Int(i),
            Literal::Bool(b) => ConstantValue::Bool(b),
            Literal::Float(f) => ConstantValue::Float(f.to_bits()),
            Literal::Char(c) => ConstantValue::Char(c),
        }
    }

    fn constant_to_literal(value: ConstantValue) -> Literal {
        match value {
            ConstantValue::Int(value) => Literal::Int(value),
            ConstantValue::Bool(bool) => Literal::Bool(bool),
            ConstantValue::Float(bits) => Literal::Float(f64::from_bits(bits)),
            ConstantValue::Char(char) => Literal::Char(char),
        }
    }

    fn assign_var_to_new_vn(table: &mut LVNTable, dest: &str) -> ValueNumber {
        let vn = table.fresh_value_number();
        table.assign_vn_to_variable(dest.to_owned(), vn);
        vn
    }

    fn create_const_instruction(
        table: &mut LVNTable,
        vn: u32,
        constant_key: ConstantKey,
        dest: &str,
        pos: &Option<Position>,
    ) -> Instruction {
        table.assign_vn_to_variable(dest.to_owned(), vn);

        Instruction::Constant {
            dest: dest.to_owned(),
            op: ConstOps::Const,
            const_type: constant_key.ty,
            value: Self::constant_to_literal(constant_key.value),
            pos: pos.clone(),
        }
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
                    value: Self::literal_to_constant(value.clone()),
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
                let mut arg_vns: Vec<ValueNumber> = args
                    .iter_mut()
                    .map(|arg| table.get_or_create_vn_from_variable(arg))
                    .collect();

                for (arg, vn) in args.iter_mut().zip(&arg_vns) {
                    if let Some(var) = table.get_variable_from_vn(*vn) {
                        *arg = var.to_owned();
                    }
                }

                // Special handling for id: Replace with Const
                if *op == ValueOps::Id {
                    // Id instruction only has one argument
                    let source_vn = arg_vns[0];

                    // Propagate constants when the source is known to be constant.
                    if let Some(constant) = table.get_constant_from_vn(&source_vn).cloned() {
                        *instr =
                            Self::create_const_instruction(table, source_vn, constant, dest, pos);
                        return;
                    }

                    table.assign_vn_to_variable(dest.clone(), source_vn);
                    return;
                }

                if !Self::is_lvn_eligible(op) {
                    Self::assign_var_to_new_vn(table, dest);
                    return;
                }

                // Find constants of arguments if there is any
                let operand_constants: Vec<Option<ConstantKey>> = arg_vns
                    .iter()
                    .map(|vn| table.get_constant_from_vn(vn).cloned())
                    .collect();

                println!("{:?}", operand_constants);

                // Comparison folding and constant folding
                let folded_constant = fold_self_comparison(op, &arg_vns)
                    .or_else(|| fold_arithmetic_operation(op, &operand_constants))
                    .or_else(|| fold_logical_operation(op, &operand_constants));

                if let Some(folded_constant) = folded_constant {
                    let vn = table.get_or_create_vn_from_constant(folded_constant.clone());
                    *instr = Self::create_const_instruction(table, vn, folded_constant, dest, pos);
                    return;
                }

                // Common Subexpression Elimination
                Self::canonicalize_expression_args(op, &mut arg_vns);

                let expression = Expression {
                    op: *op,
                    args: arg_vns,
                    op_type: op_type.clone(),
                };

                // The expression may still exist in the expression table even if all
                // variables holding its VN have since been reassigned.
                if let Some(existing_vn) = table.get_vn_from_expression(&expression) {
                    // Canonicalize operands to use the representative variable
                    // for each value number.
                    for (arg, vn) in args.iter_mut().zip(&expression.args) {
                        if let Some(var) = table.get_variable_from_vn_except(*vn, arg) {
                            *arg = var.to_owned();
                        }
                    }

                    // Only replace the computation with an Id if another variable still
                    // holds the result. Exclude dest to avoid generating `dest = id dest` when reassigning.
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
                    let vn = Self::assign_var_to_new_vn(table, dest);
                    table.insert_expression(expression, vn);
                }
            }

            Instruction::Effect { args, .. } => {
                for arg in args.iter_mut() {
                    if let Some(vn) = table.get_vn_from_variable(arg)
                        && let Some(var) = table.get_variable_from_vn_except(vn, arg)
                    {
                        *arg = var.to_owned();
                    }
                }
            }
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
