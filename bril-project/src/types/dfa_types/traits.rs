use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::types::{BasicBlock, dfa_types::Direction};

pub trait DataFlowElement: Hash + Eq + PartialEq + Display + Debug + Default {}

pub trait DataFlowDomain: Eq + Clone + PartialEq + Display + Debug {
    fn merge(&mut self, other: &Self);
}

pub trait DataFlowAnalysisTrait: Debug + Clone + Copy {
    type Domain: DataFlowDomain;

    fn init(&self) -> Self::Domain;

    fn direction(&self) -> Direction;

    fn transfer(&self, curr_block: &BasicBlock, input: &Self::Domain) -> Self::Domain;
}
