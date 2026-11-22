use bril_rs::Instruction;

use crate::{
    dfa::defined::{domain::DefinedVariables, variable::Variable},
    types::{BasicBlock, DataFlowAnalysisTrait, DataFlowSet, Direction},
};

#[derive(Default, Debug, Copy, Clone)]
pub struct DefinedVariablesAnalysis;

impl DefinedVariablesAnalysis {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    const fn get_variable(instr: &Instruction) -> Option<&str> {
        match instr {
            Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } => {
                Some(dest.as_str())
            }
            Instruction::Effect { .. } => None,
        }
    }
}

impl DataFlowAnalysisTrait for DefinedVariablesAnalysis {
    type Domain = DefinedVariables;

    fn init(&self) -> Self::Domain {
        DataFlowSet::default()
    }

    fn direction(&self) -> Direction {
        Direction::Forward
    }

    fn transfer(&self, block: &BasicBlock, input: &Self::Domain) -> Self::Domain {
        let mut output = input.clone();

        for instruction in block.get_instrs() {
            if let Some(variable) = Self::get_variable(instruction) {
                output.insert(Variable {
                    variable: variable.to_string(),
                });
            }
        }

        output
    }
}
