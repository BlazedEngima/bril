use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::types::{BasicBlock, BlockId, DataFlowResult};

/// Used to define a type as an element of a data flow analysis
/// Is usually encapsulated within a [`DataFlowDomain`]
pub trait DataFlowElement: Hash + Eq + PartialEq + Display + Debug + Default {}

pub trait DataFlowDirection {
    /// The source of information for the current block
    ///
    /// Forward: predecessors
    /// Backward: sucessors
    fn sources(block: &BasicBlock) -> &[BlockId];

    /// The target blocks to propagate information to
    ///
    /// Forward: sucessors
    /// Backward: predecessors
    fn targets(block: &BasicBlock) -> &[BlockId];

    /// Returns the collection of artifacts to read from the flow dependencies.
    ///
    /// Forward: OUT facts.
    /// Backward: IN facts.
    fn fact_dependency<D: DataFlowDomain>(result: &DataFlowResult<D>) -> &[D];

    /// Determines the initial order in which basic blocks are inserted
    /// into the worklist.
    ///
    /// Forward: Entry-to-exit order.
    /// Backward: Exit-to-entry order.
    fn initial_worklist_order(blocks: &[BasicBlock]) -> impl Iterator<Item = &BasicBlock>;

    /// Updates the current block's IN and OUT facts.
    ///
    /// Returns whether the fact relevant to propagation has changed.
    fn update_facts<D: DataFlowDomain>(
        result: &mut DataFlowResult<D>,
        block_id: BlockId,
        source: D,
        target: D,
    ) -> bool;
}

/// Defines the domain in which the data flow analysis operates
/// Is usually a container struct that stores [`DataFlowElement`]s
pub trait DataFlowDomain: Eq + Clone + PartialEq + Display + Debug {
    fn merge(&mut self, other: &Self);
}

/// Defines the data flow analysis
pub trait DataFlowAnalysisTrait<D: DataFlowDirection>: Debug + Clone + Copy {
    type Domain: DataFlowDomain;

    fn init(&self) -> Self::Domain;

    fn transfer(&self, curr_block: &BasicBlock, input: &Self::Domain) -> Self::Domain;
}
