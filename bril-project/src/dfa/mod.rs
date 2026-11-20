pub mod core;
pub mod reaching;

use std::fmt::Display;

use reaching::analysis::ReachingDefinitionsAnalysis;

use crate::{
    dfa::core::DataFlowAnalysis,
    types::{DataFlowResult, ProgramCFG},
};

#[derive(Debug, Clone)]
pub enum DFAAnalysis {
    ReachingDefinitions(ReachingDefinitionsAnalysis),
}

impl DFAAnalysis {
    pub fn run(&self, cfg: &ProgramCFG) {
        match self {
            Self::ReachingDefinitions(analysis) => {
                let results = DataFlowAnalysis::run(cfg, analysis);

                Self::print(cfg, &results);
            }
        }
    }

    fn print<D: Display>(cfg: &ProgramCFG, dfa_result: &[DataFlowResult<D>]) {
        for (idx, item) in dfa_result.iter().enumerate() {
            let func_cfg = &cfg.get_functions()[idx];
            item.print_with_header_names(func_cfg);
        }
    }
}
