use std::{
    borrow::Borrow,
    fmt::{Debug, Display, Formatter, Result},
};

use crate::types::DataFlowElement;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Variable {
    pub variable: String,
}

impl DataFlowElement for Variable {}

impl Borrow<str> for Variable {
    fn borrow(&self) -> &str {
        &self.variable
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.variable)?;

        Ok(())
    }
}
