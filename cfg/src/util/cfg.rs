use crate::util::types::{
    Block, BlockMap, BlockSuccessorMap, Label, ProgramMap, ProgramSuccessorMap,
};
use bril_rs::{Code, EffectOps, Instruction, Program};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

// If previous block has no name then assign some branch name
pub fn close_block(block_map: &mut BlockMap, block: &mut Block, counter: &mut u64) {
    if block.label.name.is_empty() {
        block.label.name = format!("block_{}", counter);
        *counter += 1;
    }

    block_map.insert(block.label.clone(), block.clone());
    block.clear();
}

pub fn process_instruction(
    block_map: &mut BlockMap,
    block: &mut Block,
    instr: Instruction,
    counter: &mut u64,
) {
    match instr {
        Instruction::Effect {
            args,
            funcs,
            labels,
            op,
            pos,
        } => {
            let effect_instr = Instruction::Effect {
                args,
                funcs,
                labels,
                op,
                pos,
            };

            block.instrs.push(effect_instr);
            // End of block instructions are jump, branch, and return
            match op {
                EffectOps::Jump | EffectOps::Branch | EffectOps::Return => {
                    close_block(block_map, block, counter);
                }

                _ => {}
            }
        }
        _ => block.instrs.push(instr),
    }
}

pub fn get_code_block(code: Vec<Code>) -> BlockMap {
    let mut block_map = IndexMap::new();
    let mut block = Block::new();
    let mut counter = 0;
    for instr in code {
        match instr {
            Code::Instruction(instr) => {
                process_instruction(&mut block_map, &mut block, instr, &mut counter)
            }
            // If block hits label, then push into the block_map
            Code::Label { label, pos } => {
                let label = Label {
                    name: label,
                    position: pos,
                };

                close_block(&mut block_map, &mut block, &mut counter);
                block.label = label;
            }
        }
    }

    // Add to block map if block has contents after parsing everything
    close_block(&mut block_map, &mut block, &mut counter);
    block_map
}

pub fn get_successors(block_map: &BlockMap) -> BlockSuccessorMap {
    let mut block_successor_map = HashMap::new();

    for (i, (label, block)) in block_map.iter().enumerate() {
        let last_instr = block
            .instrs
            .last()
            .expect("Instruction vector in Block should not be empty");
        let mut successors = HashSet::new();

        // Extract successors from last instruction in basic block
        match last_instr {
            Instruction::Effect { labels, op, .. } => {
                if *op == EffectOps::Return {
                    continue;
                }

                successors.extend(labels.clone());
            }
            Instruction::Value { labels, .. } => successors.extend(labels.clone()),
            _ => {
                // Check if the next instruction is a new basic block and add it
                if let Some((label, _)) = block_map.get_index(i + 1) {
                    successors.insert(label.name.clone());
                }
            }
        }

        block_successor_map.insert(label.clone(), successors.clone());
        successors.clear();
    }

    block_successor_map
}

pub fn get_successor_map(program_map: &ProgramMap) -> ProgramSuccessorMap {
    let mut successor_map = HashMap::new();
    for (func_name, block_map) in program_map.iter() {
        let sucessors = get_successors(block_map);

        successor_map.insert(func_name.clone(), sucessors);
    }

    successor_map
}

pub fn get_cfg(program: Program) -> ProgramMap {
    let mut program_map = IndexMap::new();
    for function in program.functions {
        let function_label = Label {
            name: function.name,
            position: function.pos,
        };

        let code_block = get_code_block(function.instrs);
        program_map.insert(function_label, code_block);
    }

    program_map
}
