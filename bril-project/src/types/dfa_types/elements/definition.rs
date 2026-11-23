use std::fmt::{Debug, Display, Formatter, Result};

use crate::types::{BlockId, DataFlowElement};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Definition {
    pub variable: String,
    pub block_id: BlockId,
    pub instruction_index: usize,
}

impl DataFlowElement for Definition {}

impl Display for Definition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.variable)?;

        Ok(())
    }
}
