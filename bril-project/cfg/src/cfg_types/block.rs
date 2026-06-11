use bril_rs::Instruction;
use std::fmt::{Debug, Display, Formatter, Result};

pub type BlockId = usize; // Block Id to index blocks in Vector

#[derive(Default, Debug, Clone)]
pub struct BasicBlock {
    id: BlockId,
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

    pub fn get_id(&self) -> BlockId {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        self.instrs.is_empty()
    }

    pub fn has_successors(&self) -> bool {
        !self.successors.is_empty()
    }

    pub fn clear(&mut self) {
        self.instrs.clear();
        self.successors.clear();
        self.id = 0;
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn insert_instr(&mut self, instr: Instruction) {
        self.instrs.push(instr);
    }

    pub fn get_instrs(&self) -> &Vec<Instruction> {
        &self.instrs
    }

    pub fn get_successors(&self) -> &Vec<BlockId> {
        &self.successors
    }

    pub fn insert_successor(&mut self, successor: BlockId) {
        self.successors.push(successor);
    }

    pub fn set_successor(&mut self, successors: Vec<BlockId>) {
        self.successors = successors;
    }
}

/// Full debugging information of Block
impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "  instructions:")?;

        for instr in &self.instrs {
            writeln!(f, "    {}", instr)?;
        }

        writeln!(f)?;
        write!(f, "  successors: [")?;

        let mut first = true;
        for id in &self.successors {
            if first {
                write!(f, "{}", id)?;
                first = false;
            } else {
                write!(f, ", {}", id)?;
            }
        }
        writeln!(f, "]")?;

        Ok(())
    }
}
