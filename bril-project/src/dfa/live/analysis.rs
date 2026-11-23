use bril_rs::Instruction;

use crate::{
    dfa::live::domain::LiveVariables,
    types::{Backward, BasicBlock, DataFlowAnalysisTrait, elements::Variable},
};

#[derive(Default, Debug, Copy, Clone)]
pub struct LiveVariablesAnalysis;

impl LiveVariablesAnalysis {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    const fn get_var_definition(instr: &Instruction) -> Option<&str> {
        match instr {
            Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } => {
                Some(dest.as_str())
            }
            Instruction::Effect { .. } => None,
        }
    }

    fn get_var_arguments(instr: &Instruction) -> &[String] {
        match instr {
            Instruction::Value { args, .. } | Instruction::Effect { args, .. } => args,
            Instruction::Constant { .. } => &[],
        }
    }
}

impl DataFlowAnalysisTrait<Backward> for LiveVariablesAnalysis {
    type Domain = LiveVariables;

    fn init(&self) -> Self::Domain {
        LiveVariables::default()
    }

    fn transfer(&self, block: &BasicBlock, output: &Self::Domain) -> Self::Domain {
        let mut input = output.clone();

        for instruction in block.get_instrs().iter().rev() {
            if let Some(variable) = Self::get_var_definition(instruction) {
                input.variables.swap_remove(variable);
            }

            for variable in Self::get_var_arguments(instruction) {
                input.variables.insert(Variable {
                    variable: variable.to_owned(),
                });
            }
        }

        input
    }
}
