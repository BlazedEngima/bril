pub mod block;
pub mod function;
pub mod label;
pub mod program;

pub use block::{BasicBlock, BlockId};
pub use function::FunctionCFG;
pub use label::Label;
pub use program::ProgramCFG;
