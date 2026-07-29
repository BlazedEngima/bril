use ahash::HashSet;
use bril_rs::Instruction;
use std::fmt::{Debug, Display, Formatter, Result};

use crate::types::BitSet;

pub type BlockId = usize; // Block Id to index blocks in Vector

/// Struct representation of a Basic Block in the control flow graph
#[derive(Default, Debug, Clone)]
pub struct BasicBlock {
    id: BlockId,              // Used for mapping for label to block id
    instrs: Vec<Instruction>, // Vector of instructions within the basic block
    successors: Vec<BlockId>, // Vector of successors (edges) to this basic block
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

    pub fn get_instrs(&self) -> &[Instruction] {
        &self.instrs
    }

    pub fn into_instrs(self) -> Vec<Instruction> {
        self.instrs
    }

    pub fn get_successors(&self) -> &[BlockId] {
        &self.successors
    }

    pub fn insert_successor(&mut self, successor: BlockId) {
        self.successors.push(successor);
    }

    pub fn set_successor(&mut self, successors: Vec<BlockId>) {
        self.successors = successors;
    }

    pub fn eliminate_dead_code(&mut self, live_vars: &mut HashSet<String>) {
        let mask = self.find_unused_defs(live_vars);
        self.delete_instructions(mask);
    }

    fn delete_instructions(&mut self, mask: BitSet) {
        let mut idx = 0;
        self.instrs.retain(|_| {
            let keep = mask.is_keep(idx);
            idx += 1;
            keep
        });
    }

    fn find_unused_defs(&self, live_vars: &mut HashSet<String>) -> BitSet {
        live_vars.clear();
        let mut mask = BitSet::new_all_false(self.instrs.len());

        // Iterate backwards to get the usage first before the definition.
        for (idx, instr) in self.instrs.iter().enumerate().rev() {
            match instr {
                Instruction::Effect { args, .. } => {
                    mask.mark_keep(idx);
                    for arg in args {
                        live_vars.insert(arg.clone());
                    }
                }

                Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } => {
                    let dest_str = dest.as_str();

                    // Redefinition occurs
                    if live_vars.contains(dest_str) {
                        mask.mark_keep(idx);

                        // `dest` is re-defined here, so earlier assignments to `dest` are killed
                        live_vars.remove(dest_str);

                        // If it has argument dependencies, those args are now live
                        if let Instruction::Value { args, .. } = instr {
                            for arg in args {
                                live_vars.insert(arg.clone());
                            }
                        }
                    }
                }
            }
        }

        mask
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
