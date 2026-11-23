pub mod core;
pub mod defined;
pub mod live;
pub mod reaching;

use crate::{
    dfa::{
        core::DataFlowAnalysis, defined::analysis::DefinedVariablesAnalysis,
        live::analysis::LiveVariablesAnalysis, reaching::analysis::ReachingDefinitionsAnalysis,
    },
    types::{DataFlowAnalysisTrait, DataFlowDirection, ProgramCFG},
};

#[derive(Debug, Clone)]
pub enum DFAPass {
    ReachingDefinitions(ReachingDefinitionsAnalysis),
    DefinedVariables(DefinedVariablesAnalysis),
    LiveVariables(LiveVariablesAnalysis),
}

impl DFAPass {
    pub fn run(&self, cfg: &ProgramCFG) {
        match self {
            Self::ReachingDefinitions(analysis) => Self::execute(cfg, analysis),
            Self::DefinedVariables(analysis) => Self::execute(cfg, analysis),
            Self::LiveVariables(analysis) => Self::execute(cfg, analysis),
        }
    }

    fn execute<T, D>(cfg: &ProgramCFG, analysis: &T)
    where
        D: DataFlowDirection,
        T: DataFlowAnalysisTrait<D>,
    {
        let results = DataFlowAnalysis::run(cfg, analysis);
        DataFlowAnalysis::print(cfg, &results);
    }
}
