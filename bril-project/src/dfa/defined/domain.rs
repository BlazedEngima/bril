use crate::{
    dfa::defined::variable::Variable,
    types::{DataFlowDomain, DataFlowSet},
};

pub type DefinedVariables = DataFlowSet<Variable>;

impl DataFlowDomain for DefinedVariables {
    fn merge(&mut self, other: &Self) {
        self.extend(other.iter().cloned());
    }
}
