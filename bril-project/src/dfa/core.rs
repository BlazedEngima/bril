use std::{collections::VecDeque, fmt::Display};

use ahash::HashSet;

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
            in_facts: Vec::default(),
            out_facts: Vec::default(),
        };

        let mut worklist = VecDeque::new();
        let mut queued = HashSet::default();

        Self::init(func, analysis, &mut result, &mut worklist, &mut queued);

        while let Some(block_id) = worklist.pop_front() {
            queued.remove(&block_id);

            let block = func
                .get_block(block_id)
                .expect("Found block id with no matching basic block.");

            // Calculate IN[B].
            let input = Self::merge_predecessors(&result, analysis, block);

            // Calculate OUT[B].
            let output = analysis.transfer(block, &input);
            let output_changed = result.out_facts[block_id] != output;

            result.in_facts[block_id] = input;
            result.out_facts[block_id] = output;
            if output_changed {
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
    fn init<T: DataFlowAnalysisTrait>(
        func: &FunctionCFG,
        analysis: &T,
        result: &mut DataFlowResult<T::Domain>,
        worklist: &mut VecDeque<BlockId>,
        queued: &mut HashSet<BlockId>,
    ) {
        for block in func.get_blocks() {
            let init = analysis.init();

            result.in_facts.push(init.clone());
            result.out_facts.push(init);

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

        for &predecessor in block.get_predecessors() {
            if let Some(output) = result.out_facts.get(predecessor) {
                input.merge(output);
            }
        }

        input
    }

    pub fn print<D: Display>(cfg: &ProgramCFG, dfa_result: &[DataFlowResult<D>]) {
        for (idx, item) in dfa_result.iter().enumerate() {
            let func_cfg = &cfg.get_functions()[idx];
            item.print_with_header_names(func_cfg);
        }
    }
}
