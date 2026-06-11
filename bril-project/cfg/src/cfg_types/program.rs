use crate::cfg_types::function::FunctionCFG;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Clone)]
pub struct ProgramCFG {
    functions: Vec<FunctionCFG>,
}

/// Full debugging information of Program
impl Debug for ProgramCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for func in self.functions.iter() {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

impl Display for ProgramCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for func in self.functions.iter() {
            writeln!(f, "{}", func)?;
        }

        Ok(())
    }
}
