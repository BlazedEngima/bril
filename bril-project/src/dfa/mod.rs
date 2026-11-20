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

                Self::print(&results);
            }
        }
    }

    fn print<D: Display>(dfa_result: &Vec<DataFlowResult<D>>) {
        for item in dfa_result {
            println!("{item}");
        }
    }
}
