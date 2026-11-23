use crate::types::{DataFlowDomain, DataFlowSet, elements::Definition};

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ReachingDefinitions {
    pub definitions: DataFlowSet<Definition>,
}

impl DataFlowDomain for ReachingDefinitions {
    fn merge(&mut self, other: &Self) {
        self.definitions.extend(other.definitions.iter().cloned());
    }
}

impl Display for ReachingDefinitions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.definitions)?;
        Ok(())
    }
}
