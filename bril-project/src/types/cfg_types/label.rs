use bril_rs::Position;
use std::{
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
