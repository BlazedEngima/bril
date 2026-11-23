use std::{collections::VecDeque, fmt::Display};

use ahash::HashSet;

use crate::types::{
    BasicBlock, BlockId, DataFlowAnalysisTrait, DataFlowDirection, DataFlowDomain, DataFlowResult,
    FunctionCFG, ProgramCFG,
};

pub struct DataFlowAnalysis;

impl DataFlowAnalysis {
    pub fn run<T, D>(cfg: &ProgramCFG, analysis: &T) -> Vec<DataFlowResult<T::Domain>>
    where
        D: DataFlowDirection,
        T: DataFlowAnalysisTrait<D>,
    {
        let mut result = Vec::new();

        for func in cfg.get_functions() {
            result.push(Self::analyze(func, analysis));
        }

        result
    }

    fn analyze<T, D>(func: &FunctionCFG, analysis: &T) -> DataFlowResult<T::Domain>
    where
        D: DataFlowDirection,
        T: DataFlowAnalysisTrait<D>,
    {
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

            // Calculate:
            // Direction::Forward => IN[B]
            // Direction::Bakcward => OUT[B]
            let source = Self::merge(&result, analysis, block);

            // Calculate with the transfer function:
            // Direction::Forward => OUT[B]
            // Direction::Bakcward => IN[B]
            let target = analysis.transfer(block, &source);

            let output_changed = D::update_facts(&mut result, block_id, source, target);

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
    fn init<T, D>(
        func: &FunctionCFG,
        analysis: &T,
        result: &mut DataFlowResult<T::Domain>,
        worklist: &mut VecDeque<BlockId>,
        queued: &mut HashSet<BlockId>,
    ) where
        D: DataFlowDirection,
        T: DataFlowAnalysisTrait<D>,
    {
        for block in D::initial_worklist_order(func.get_blocks()) {
            let init = analysis.init();

            result.in_facts.push(init.clone());
            result.out_facts.push(init);

            worklist.push_back(block.get_id());
            queued.insert(block.get_id());
        }
    }

    /// Merge all neighbors of a block depending on direction
    /// Forward:
    /// <code>in_facts[block_id] = merge(out_facts[p] for every predecessor p of block)</code>
    ///
    /// Backward:
    /// <code>out_facts[block_id] = merge(in_facts[p] for every successor p of block)</code>
    fn merge<T, D>(
        result: &DataFlowResult<T::Domain>,
        analysis: &T,
        block: &BasicBlock,
    ) -> T::Domain
    where
        D: DataFlowDirection,
        T: DataFlowAnalysisTrait<D>,
    {
        let mut input = analysis.init();
        let facts = D::fact_dependency(result);

        for &neighbor in D::sources(block) {
            if let Some(output) = facts.get(neighbor) {
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
