use bril_rs::Instruction;

use crate::{
    dfa::reaching::{definition::Definition, domain::ReachingDefinitions},
    types::{BasicBlock, DataFlowAnalysisTrait, DataFlowSet, Direction},
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

impl DataFlowAnalysisTrait for ReachingDefinitionsAnalysis {
    type Domain = ReachingDefinitions;

    fn init(&self) -> Self::Domain {
        DataFlowSet::default()
    }

    fn direction(&self) -> Direction {
        Direction::Forward
    }

    fn transfer(&self, block: &BasicBlock, input: &Self::Domain) -> Self::Domain {
        let mut output = input.clone();

        for (index, instruction) in block.get_instrs().iter().enumerate() {
            let Some(variable) = Self::defined_variable(instruction) else {
                continue;
            };

            // Kill all previous definitions of this variable.
            output.retain(|def| def.variable != variable);

            output.insert(Definition {
                variable: variable.to_string(),
                block_id: block.get_id(),
                instruction_index: index,
            });
        }

        output
    }
}
