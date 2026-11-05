mod block;
mod function;
mod label;
mod program;

pub use block::{BasicBlock, BlockId};
pub use function::FunctionCFG;
pub use label::Label;
pub use program::ProgramCFG;
