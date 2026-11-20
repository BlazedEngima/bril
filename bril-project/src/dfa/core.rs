use std::collections::{VecDeque, hash_map::Entry};

use ahash::{HashMap, HashSet};

use crate::types::{
    BasicBlock, BlockId, DataFlowAnalysisTrait, DataFlowDomain, DataFlowResult, FunctionCFG,
    ProgramCFG,
};

pub struct DataFlowAnalysis;

impl DataFlowAnalysis {
    pub fn run<T: DataFlowAnalysisTrait>(
        cfg: &ProgramCFG,
        analysis: &T,
    ) -> Vec<DataFlowResult<T::Domain>> {
        let mut result = Vec::new();

        for func in cfg.get_functions() {
            result.push(Self::analyze(func, analysis));
        }

        result
    }

    fn analyze<T: DataFlowAnalysisTrait>(
        func: &FunctionCFG,
        analysis: &T,
    ) -> DataFlowResult<T::Domain> {
        let mut result = DataFlowResult {
            in_facts: HashMap::default(),
            out_facts: HashMap::default(),
        };

        let mut worklist = VecDeque::new();
        let mut queued = HashSet::default();

        Self::init(func, &mut worklist, &mut queued);

        while let Some(block_id) = worklist.pop_front() {
            queued.remove(&block_id);

            let block = func
                .get_block(block_id)
                .expect("Found block id with no matching basic block.");

            // Calculate IN[B].
            let input = Self::merge_predecessors(&result, analysis, block);

            // Calculate OUT[B].
            let output = analysis.transfer(block, &input);

            result.in_facts.insert(block_id, input);
            if Self::update_output_and_check_changed(&mut result, block_id, output) {
                for &successor in block.get_successors() {
                    if queued.insert(successor) {
                        worklist.push_back(successor);
                    }
                }
            }
        }
        result
    }

    /// Initialize the in and out artifacts for the results and the worklist
    fn init(func: &FunctionCFG, worklist: &mut VecDeque<BlockId>, queued: &mut HashSet<BlockId>) {
        for block in func.get_blocks() {
            worklist.push_back(block.get_id());
            queued.insert(block.get_id());
        }
    }

    /// Merge all predecessors of a block
    /// <code>in_facts[block_id] = merge(out_facts[p] for every predecessor p of block)</code>
    fn merge_predecessors<T: DataFlowAnalysisTrait>(
        result: &DataFlowResult<T::Domain>,
        analysis: &T,
        block: &BasicBlock,
    ) -> T::Domain {
        let mut input = analysis.init();

        for predecessor in block.get_predecessors() {
            if let Some(output) = result.out_facts.get(predecessor) {
                input.merge(output);
            }
        }

        input
    }

    /// Updates output and returns boolean if output has changed or not
    fn update_output_and_check_changed<D: DataFlowDomain>(
        result: &mut DataFlowResult<D>,
        block_id: BlockId,
        output: D,
    ) -> bool {
        match result.out_facts.entry(block_id) {
            Entry::Occupied(mut entry) => {
                if entry.get() == &output {
                    false
                } else {
                    entry.insert(output);
                    true
                }
            }

            Entry::Vacant(entry) => {
                entry.insert(output);
                true
            }
        }
    }
}
