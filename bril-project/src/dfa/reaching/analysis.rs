use bril_rs::Instruction;

use crate::{
    dfa::reaching::domain::ReachingDefinitions,
    types::{BasicBlock, DataFlowAnalysisTrait, Forward, elements::Definition},
};

#[derive(Default, Debug, Copy, Clone)]
pub struct ReachingDefinitionsAnalysis;

impl ReachingDefinitionsAnalysis {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    const fn defined_variable(instr: &Instruction) -> Option<&str> {
        match instr {
            Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } => {
                Some(dest.as_str())
            }
            Instruction::Effect { .. } => None,
        }
    }
}

impl DataFlowAnalysisTrait<Forward> for ReachingDefinitionsAnalysis {
    type Domain = ReachingDefinitions;

    fn init(&self) -> Self::Domain {
        ReachingDefinitions::default()
    }

    fn transfer(&self, block: &BasicBlock, input: &Self::Domain) -> Self::Domain {
        let mut output = input.clone();

        for (index, instruction) in block.get_instrs().iter().enumerate() {
            let Some(variable) = Self::defined_variable(instruction) else {
                continue;
            };

            // Kill all previous definitions of this variable.
            output.definitions.retain(|def| def.variable != variable);

            output.definitions.insert(Definition {
                variable: variable.to_string(),
                block_id: block.get_id(),
                instruction_index: index,
            });
        }

        output
    }
}
