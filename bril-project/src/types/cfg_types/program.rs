use crate::types::FunctionCFG;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default, Clone)]
pub struct ProgramCFG {
    functions: Vec<FunctionCFG>,
}

impl ProgramCFG {
    pub fn insert_function_cfg(&mut self, function_cfg: FunctionCFG) {
        self.functions.push(function_cfg);
    }

    pub fn get_functions(&self) -> &[FunctionCFG] {
        &self.functions
    }

    pub fn get_functions_mut(&mut self) -> &mut [FunctionCFG] {
        &mut self.functions
    }
}

/// Full debugging information of Program
impl Debug for ProgramCFG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for func in self.functions.iter() {
            writeln!(f, "{:?}", func)?;
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
