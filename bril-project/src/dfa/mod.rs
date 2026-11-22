pub mod core;
pub mod defined;
pub mod reaching;

use reaching::analysis::ReachingDefinitionsAnalysis;

use crate::{
    dfa::{core::DataFlowAnalysis, defined::analysis::DefinedVariablesAnalysis},
    types::{DataFlowAnalysisTrait, ProgramCFG},
};

#[derive(Debug, Clone)]
pub enum DFAPass {
    ReachingDefinitions(ReachingDefinitionsAnalysis),
    DefinedVariables(DefinedVariablesAnalysis),
}

impl DFAPass {
    pub fn run(&self, cfg: &ProgramCFG) {
        match self {
            Self::ReachingDefinitions(analysis) => Self::execute(cfg, analysis),
            Self::DefinedVariables(analysis) => Self::execute(cfg, analysis),
        }
    }

    fn execute<T: DataFlowAnalysisTrait>(cfg: &ProgramCFG, analysis: &T) {
        let results = DataFlowAnalysis::run(cfg, analysis);
        DataFlowAnalysis::print(cfg, &results);
    }
}
