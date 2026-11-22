use std::fmt::{Debug, Display, Formatter, Result};

use crate::types::DataFlowElement;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Variable {
    pub variable: String,
}

impl DataFlowElement for Variable {}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.variable)?;

        Ok(())
    }
}
