use std::str::FromStr;

use crate::dfa::{DFAAnalysis, reaching::analysis::ReachingDefinitionsAnalysis};

#[derive(Debug, Default)]
pub struct Modules {
    pub dump_cfg: bool,
    pub tdce: bool,
    pub lvn: bool,
    pub dfa: Option<DFAAnalysis>,
}

// Convert module arguments into active flags.
//
// Short forms: -m ctl
// Long forms:  -m cfg,tdce,lvn,dfa:reaching
impl FromStr for Modules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = Self::default();

        // Handle compact short forms.
        if !s.contains(',') && s.chars().all(|ch| matches!(ch, 'c' | 't' | 'l')) {
            for ch in s.chars() {
                match ch {
                    'c' => modules.dump_cfg = true,
                    't' => modules.tdce = true,
                    'l' => modules.lvn = true,
                    _ => unreachable!(),
                }
            }

            return Ok(modules);
        }

        // Handle comma-separated module names.
        for module in s.split(',') {
            match module.trim() {
                "c" | "cfg" => modules.dump_cfg = true,
                "t" | "tdce" => modules.tdce = true,
                "l" | "lvn" => modules.lvn = true,

                "reaching" | "dfa:reaching" | "dfa:reaching-definitions" => {
                    if modules.dfa.is_some() {
                        return Err("Only one DFA analysis can be specified.".into());
                    }

                    modules.dfa = Some(DFAAnalysis::ReachingDefinitions(
                        ReachingDefinitionsAnalysis::new(),
                    ));
                }

                unknown => {
                    return Err(format!("Unknown module: '{unknown}'"));
                }
            }
        }

        Ok(modules)
    }
}
