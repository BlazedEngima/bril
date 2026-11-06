use ahash::HashSet;

use crate::types::{FunctionCFG, ProgramCFG};

pub struct DeadCodeElimination;

impl DeadCodeElimination {
    fn eliminate_func_dead_code(function: &mut FunctionCFG) {
        let mut live_vars = HashSet::default();

        for basic_block in function.get_blocks_mut() {
            basic_block.eliminate_dead_code(&mut live_vars);
        }
    }

    pub fn run(program: &mut ProgramCFG) {
        for function in program.get_functions_mut() {
            Self::eliminate_func_dead_code(function);
        }
    }
}
