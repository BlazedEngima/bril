use bril_rs::Instruction;

use crate::types::{BasicBlock, BlockId, Label};
use ahash::HashMap;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Clone)]
pub struct FunctionCFG {
    name: Label,
    blocks: Vec<BasicBlock>,
    block_id_to_label: HashMap<BlockId, Label>,
}

impl FunctionCFG {
    pub fn new(name: Label) -> Self {
        Self {
            name,
            blocks: Vec::default(),
            block_id_to_label: HashMap::default(),
        }
    }

    pub fn insert_block_id(&mut self, id: BlockId, name: Label) {
        self.block_id_to_label.insert(id, name);
    }

    pub fn insert_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    pub fn has_id(&self, id: &BlockId) -> bool {
        self.block_id_to_label.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn build_sucessors(&mut self, label_to_block_id: &HashMap<String, BlockId>) {
        let num_blocks = self.blocks.len();
        for (idx, block) in self.blocks.iter_mut().enumerate() {
            let successors = Self::compute_sucessors(idx, num_blocks, block, label_to_block_id);
            block.set_successor(successors);
        }
    }

    fn compute_sucessors(
        idx: usize,
        num_blocks: usize,
        block: &BasicBlock,
        label_to_block_id: &HashMap<String, BlockId>,
    ) -> Vec<BlockId> {
        let last_instr = block
            .get_instrs()
            .last()
            .expect("Instruction vector in Block should not be empty");

        // Extract successors from last instruction in basic block
        match last_instr {
            // Not sure if a Value instruction will ever be a terminating instruction in a basic
            // block or why theres a labels field in Instruction. Will leave it here for now
            // until I find enough reason to justify removing it.
            Instruction::Effect { labels, .. } | Instruction::Value { labels, .. } => labels
                .iter()
                .map(|label| {
                    *label_to_block_id
                        .get(label)
                        .expect("No label found in label to block id map")
                })
                .collect(),
            // If the instruction is not the last instruction, add successor edge of the
            // next instruction in the CFG
            _ if idx + 1 < num_blocks => vec![idx + 1],
            _ => vec![],
        }
    }
}

/// Full debugging information of Function
impl Debug for FunctionCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (id, block) in self.blocks.iter().enumerate() {
            match self.block_id_to_label.get(&id) {
                Some(label) => writeln!(f, "BB{} ({}):", id, label)?,
                None => writeln!(f, "BB{}:", id)?,
            }
            writeln!(f, "{}", block)?;
        }
        Ok(())
    }
}

/// Graphviz representation of Function for visualization
impl Display for FunctionCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "digraph {} {{", self.name)?;
        writeln!(f, "    node [shape=box];")?;
        writeln!(f)?;

        // nodes
        for (id, block) in self.blocks.iter().enumerate() {
            if let Some(label) = self.block_id_to_label.get(&id) {
                write!(f, "    {} [label=\"", label)?;
            } else {
                write!(f, "    BB{} [label=\"", id)?;
            }

            for instr in block.get_instrs() {
                write!(f, "{}\\l", instr)?;
            }

            writeln!(f, "\"];")?;
        }

        // edges
        for (id, block) in self.blocks.iter().enumerate() {
            for succ in block.get_successors() {
                // Print the current block
                if let Some(label) = self.block_id_to_label.get(&id) {
                    write!(f, "    {} -> ", label)?;
                } else {
                    write!(f, "    BB{} -> ", id)?;
                }

                // Print the successors
                if let Some(label) = self.block_id_to_label.get(succ) {
                    writeln!(f, "{};", label)?;
                } else {
                    writeln!(f, "BB{};", succ)?;
                }
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}
