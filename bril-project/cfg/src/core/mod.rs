mod cfg;
mod print;

pub use cfg::{
    close_block, get_cfg, get_code_block, get_successor_map, get_successors, process_instruction,
};

pub use print::print_graphviz;
