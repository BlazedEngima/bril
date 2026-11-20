use crate::{
    dfa::reaching::definition::Definition,
    types::{DataFlowDomain, DataFlowSet},
};

pub type ReachingDefinitions = DataFlowSet<Definition>;

impl DataFlowDomain for ReachingDefinitions {
    fn merge(&mut self, other: &Self) {
        self.extend(other.iter().cloned());
    }
}
