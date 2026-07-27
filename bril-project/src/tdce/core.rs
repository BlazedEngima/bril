use ahash::HashSet;

use crate::types::{FunctionCFG, ProgramCFG};

fn eliminate_func_dead_code(function: &mut FunctionCFG) {
    let mut live_vars = HashSet::default();

    for basic_block in function.get_blocks_mut() {
        basic_block.eliminate_dead_code(&mut live_vars);
    }
}

pub fn eliminate_dead_code(program: &mut ProgramCFG) {
    for function in program.get_functions_mut() {
        eliminate_func_dead_code(function);
    }
}
