use bril_rs::Instruction;
use std::fmt::{Debug, Display, Formatter, Result};

pub type BlockId = usize; // Block Id to index blocks in Vector

#[derive(Default, Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instrs: Vec<Instruction>,
    pub successors: Vec<BlockId>,
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            instrs: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.instrs.clear();
        self.successors.clear();
    }
}

/// Full debugging information of Block
impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "\tinstructions:")?;

        for instr in &self.instrs {
            writeln!(f, "\t\t{}", instr)?;
        }

        writeln!(f, "\tsuccessors: {:?}", self.successors)?;
        writeln!(f)?;
        Ok(())
    }
}
