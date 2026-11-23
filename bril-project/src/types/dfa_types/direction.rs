use crate::types::{
    BasicBlock, BlockId, DataFlowDomain, DataFlowResult, dfa_types::traits::DataFlowDirection,
};

pub struct Forward;
pub struct Backward;

/// Defines the forward flow of the information
/// Check [`DataFlowDirection`] for details
impl DataFlowDirection for Forward {
    fn sources(block: &BasicBlock) -> &[BlockId] {
        block.get_predecessors()
    }

    fn targets(block: &BasicBlock) -> &[BlockId] {
        block.get_successors()
    }

    fn fact_dependency<D: DataFlowDomain>(result: &DataFlowResult<D>) -> &[D] {
        &result.out_facts
    }

    fn initial_worklist_order(blocks: &[BasicBlock]) -> impl Iterator<Item = &BasicBlock> {
        blocks.iter()
    }

    fn update_facts<D: DataFlowDomain>(
        result: &mut DataFlowResult<D>,
        block_id: BlockId,
        source: D,
        target: D,
    ) -> bool {
        let changed = result.out_facts[block_id] != target;

        result.in_facts[block_id] = source;
        result.out_facts[block_id] = target;

        changed
    }
}

/// Defines the backward flow of the information
/// Check [`DataFlowDirection`] for details
impl DataFlowDirection for Backward {
    fn sources(block: &BasicBlock) -> &[BlockId] {
        block.get_successors()
    }

    fn targets(block: &BasicBlock) -> &[BlockId] {
        block.get_predecessors()
    }

    fn fact_dependency<D: DataFlowDomain>(result: &DataFlowResult<D>) -> &[D] {
        &result.in_facts
    }

    fn initial_worklist_order(blocks: &[BasicBlock]) -> impl Iterator<Item = &BasicBlock> {
        blocks.iter().rev()
    }

    fn update_facts<D: DataFlowDomain>(
        result: &mut DataFlowResult<D>,
        block_id: BlockId,
        source: D,
        target: D,
    ) -> bool {
        let changed = result.in_facts[block_id] != target;

        result.out_facts[block_id] = source;
        result.in_facts[block_id] = target;

        changed
    }
}
