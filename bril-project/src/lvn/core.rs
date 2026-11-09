use bril_rs::{ConstOps, Instruction, Literal, Position, ValueOps};

use crate::{
    lvn::{fold_const_arithmetic_operation, fold_const_logical_operation, fold_self_comparison},
    types::{
        BasicBlock, ConstantKey, ConstantValue, Expression, LVNTable, ProgramCFG, ValueNumber,
    },
};

pub struct LocalValueNumbering;

impl LocalValueNumbering {
    pub fn run(program: &mut ProgramCFG) {
        for function in program.get_functions_mut() {
            for basic_block in function.get_blocks_mut() {
                Self::process_block(basic_block);
            }
        }
    }

    fn process_block(block: &mut BasicBlock) {
        let mut table = LVNTable::new();

        for instr in block.get_instrs_mut() {
            Self::process_instruction(instr, &mut table);
        }
    }

    fn process_instruction(instr: &mut Instruction, table: &mut LVNTable) {
        match instr {
            Instruction::Constant { .. } => {
                Self::process_constant(instr, table);
            }

            Instruction::Value { .. } => {
                Self::process_value(instr, table);
            }

            Instruction::Effect { .. } => {
                Self::process_effect(instr, table);
            }
        }
    }

    fn process_constant(instr: &mut Instruction, table: &mut LVNTable) {
        let Instruction::Constant { dest, value, .. } = instr else {
            unreachable!()
        };

        let key = ConstantKey {
            value: Self::literal_to_constant(value),
        };

        let vn = table.get_or_create_vn_from_constant(key);
        table.assign_vn_to_variable(dest.clone(), vn);
    }

    fn process_value(instr: &mut Instruction, table: &mut LVNTable) {
        let Instruction::Value {
            op,
            dest,
            args,
            op_type,
            pos,
            ..
        } = instr
        else {
            unreachable!()
        };

        // Resolve operand value numbers.
        let mut arg_vns: Vec<ValueNumber> = args
            .iter()
            .map(|arg| table.get_or_create_vn_from_variable(arg))
            .collect();

        // Canonicalize operands using their representative variables.
        Self::rewrite_operands(args, &arg_vns, table);

        if *op == ValueOps::Id {
            Self::process_id(instr, table);
            return;
        }

        // Operations outside LVN's scope just get a value number.
        if !Self::is_lvn_eligible(*op) {
            Self::assign_fresh_value_number(table, dest);
            return;
        }

        // Attempt constant folding.
        let folded_constant = fold_self_comparison(*op, &arg_vns)
            .or_else(|| fold_const_arithmetic_operation(*op, &arg_vns, table))
            .or_else(|| fold_const_logical_operation(*op, &arg_vns, table));

        if let Some(folded_constant) = folded_constant {
            let vn = table.get_or_create_vn_from_constant(folded_constant.clone());
            *instr = Self::create_const_instruction(table, vn, &folded_constant, dest, pos.clone());
            return;
        }

        // Common subexpression elimination.
        Self::sort_expression_args(*op, &mut arg_vns);

        let expression = Expression {
            op: *op,
            args: arg_vns,
            op_type: op_type.clone(),
        };

        if let Some(existing_vn) = table.get_vn_from_expression(&expression) {
            Self::rewrite_cse_instruction(instr, existing_vn, table);
        } else {
            let vn = Self::assign_fresh_value_number(table, dest);
            table.insert_expression(expression, vn);
        }
    }

    fn process_effect(instr: &mut Instruction, table: &LVNTable) {
        let Instruction::Effect { args, .. } = instr else {
            unreachable!()
        };

        for arg in args.iter_mut() {
            if let Some(vn) = table.get_vn_from_variable(arg)
                && let Some(var) = table.get_repr_variable_from_vn(vn)
            {
                var.clone_into(arg);
            }
        }
    }

    fn process_id(instr: &mut Instruction, table: &mut LVNTable) {
        let Instruction::Value {
            dest, args, pos, ..
        } = instr
        else {
            unreachable!()
        };

        let source_vn = table.get_or_create_vn_from_variable(&args[0]);

        if let Some(constant) = table.get_constant_from_vn(&source_vn).cloned() {
            *instr = Self::create_const_instruction(table, source_vn, &constant, dest, pos.clone());
            return;
        }

        table.assign_vn_to_variable(dest.clone(), source_vn);
    }

    fn rewrite_cse_instruction(
        instr: &mut Instruction,
        existing_vn: ValueNumber,
        table: &mut LVNTable,
    ) {
        let Instruction::Value {
            op,
            dest,
            args,
            funcs,
            labels,
            ..
        } = instr
        else {
            unreachable!()
        };

        // Reduce the operation to an Id operation if another variable holds its result.
        if let Some(var) = table.get_variable_from_vn_except(existing_vn, dest) {
            *op = ValueOps::Id;

            args.clear();
            args.push(var.to_owned());

            funcs.clear();
            labels.clear();
        }

        table.assign_vn_to_variable(dest.clone(), existing_vn);
    }

    fn rewrite_operands(args: &mut [String], value_numbers: &[ValueNumber], table: &LVNTable) {
        for (arg, &vn) in args.iter_mut().zip(value_numbers) {
            if let Some(variable) = table.get_repr_variable_from_vn(vn) {
                variable.clone_into(arg);
            }
        }
    }

    const fn is_lvn_eligible(op: ValueOps) -> bool {
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

    fn sort_expression_args(op: ValueOps, args: &mut [ValueNumber]) {
        if Self::is_commutative(op) {
            args.sort_unstable();
        }
    }

    const fn is_commutative(op: ValueOps) -> bool {
        matches!(
            op,
            ValueOps::Add | ValueOps::Mul | ValueOps::Eq | ValueOps::And | ValueOps::Or
        )
    }

    fn create_const_instruction(
        table: &mut LVNTable,
        vn: u32,
        constant_key: &ConstantKey,
        dest: &str,
        pos: Option<Position>,
    ) -> Instruction {
        table.assign_vn_to_variable(dest.to_owned(), vn);

        Instruction::Constant {
            dest: dest.to_owned(),
            op: ConstOps::Const,
            const_type: constant_key.ty(),
            value: Self::constant_to_literal(&constant_key.value),
            pos,
        }
    }

    const fn literal_to_constant(literal: &Literal) -> ConstantValue {
        match literal {
            Literal::Int(i) => ConstantValue::Int(*i),
            Literal::Bool(b) => ConstantValue::Bool(*b),
            Literal::Float(f) => ConstantValue::Float(f.to_bits()),
            Literal::Char(c) => ConstantValue::Char(*c),
        }
    }

    const fn constant_to_literal(value: &ConstantValue) -> Literal {
        match value {
            ConstantValue::Int(value) => Literal::Int(*value),
            ConstantValue::Bool(bool) => Literal::Bool(*bool),
            ConstantValue::Float(bits) => Literal::Float(f64::from_bits(*bits)),
            ConstantValue::Char(char) => Literal::Char(*char),
        }
    }

    fn assign_fresh_value_number(table: &mut LVNTable, dest: &str) -> ValueNumber {
        let vn = table.fresh_value_number();
        table.assign_vn_to_variable(dest.to_owned(), vn);
        vn
    }
}
