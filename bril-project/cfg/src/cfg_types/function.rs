use crate::cfg_types::{BasicBlock, Label};
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Clone)]
pub struct FunctionCFG {
    name: Label,
    blocks: Vec<BasicBlock>,
}

/// Full debugging information of Function
impl Debug for FunctionCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (id, block) in self.blocks.iter().enumerate() {
            writeln!(f, "BB{}:", id)?;
            writeln!(f, "{:?}", block)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Graphviz representation of Function for visualization
impl Display for FunctionCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "digraph {} {{", self.name)?;

        // nodes
        for (id, block) in self.blocks.iter().enumerate() {
            write!(f, "  BB{} [label=\"", id)?;

            for instr in &block.instrs {
                write!(f, "{}\\l", instr)?;
            }

            write!(f, "\"];\n")?;
        }

        // edges
        for (id, block) in self.blocks.iter().enumerate() {
            for succ in &block.successors {
                writeln!(f, "  BB{} -> BB{};", id, succ)?;
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}
