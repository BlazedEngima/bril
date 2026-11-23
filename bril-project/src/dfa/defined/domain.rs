use crate::types::{DataFlowDomain, DataFlowSet, elements::Variable};

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct DefinedVariables {
    pub variables: DataFlowSet<Variable>,
}

impl DataFlowDomain for DefinedVariables {
    fn merge(&mut self, other: &Self) {
        self.variables.extend(other.variables.iter().cloned());
    }
}

impl Display for DefinedVariables {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.variables)?;
        Ok(())
    }
}
