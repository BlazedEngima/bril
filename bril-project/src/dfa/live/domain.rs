use crate::types::{DataFlowDomain, DataFlowSet, elements::Variable};

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LiveVariables {
    pub variables: DataFlowSet<Variable>,
}

impl DataFlowDomain for LiveVariables {
    fn merge(&mut self, other: &Self) {
        self.variables.extend(other.variables.iter().cloned());
    }
}

impl Display for LiveVariables {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.variables)?;
        Ok(())
    }
}
