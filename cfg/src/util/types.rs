use bril_rs::{Instruction, Position};
use indexmap::IndexMap;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter, Result},
    hash::{Hash, Hasher},
};

/// Essentially the name of any block of instructions
/// Can also be the name of a function or code block
#[derive(Eq, PartialEq, Default, Clone)]
pub struct Label {
    pub name: String,
    pub position: Option<Position>,
}

#[derive(Default, Clone)]
pub struct Block {
    pub label: Label, // label name
    pub instrs: Vec<Instruction>,
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)?;
        Ok(())
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)?;
        Ok(())
    }
}

impl Hash for Label {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            label: Label::default(),
            instrs: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.label = Label::default();
        self.instrs.clear();
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for instr in &self.instrs {
            write!(f, "{}", instr)?;
        }
        Ok(())
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for instr in &self.instrs {
            write!(f, "\t{}", instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub type BlockMap = IndexMap<Label, Block>; // Container for individual code block
pub type ProgramMap = IndexMap<Label, BlockMap>; // Container for functions including their code blocks
pub type BlockSuccessorMap = HashMap<Label, HashSet<String>>; // Container for the successor map of a code block (BlockLabel -> SucessorLabels)
pub type ProgramSuccessorMap = HashMap<Label, BlockSuccessorMap>; // Container for the successor map of the whole program (Func -> BlockSucessorMap)
