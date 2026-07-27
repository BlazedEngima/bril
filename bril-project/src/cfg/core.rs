use crate::types::{BasicBlock, BlockId, FunctionCFG, Label, ProgramCFG};
use ahash::HashMap;
use bril_rs::{Code, EffectOps, Instruction, Program};

// If previous block has no name then assign some branch name
fn close_block(
    function_cfg: &mut FunctionCFG,
    block: &mut BasicBlock,
    id_counter: &mut BlockId,
    label_to_block_id: &mut HashMap<String, BlockId>,
) {
    if block.is_empty() {
        return;
    }

    // When encountering a named label, this will be skipped
    // as it will insert the block metadata right away
    if !function_cfg.has_id(id_counter) {
        let label_name = format!("BB{}", id_counter);
        label_to_block_id.insert(label_name, *id_counter);
    }

    block.set_id(*id_counter);
    *id_counter += 1;

    function_cfg.insert_block(block.clone());
    block.clear();
}

fn process_instruction(
    function_cfg: &mut FunctionCFG,
    block: &mut BasicBlock,
    instr: Instruction,
    id_counter: &mut BlockId,
    label_to_block_id: &mut HashMap<String, BlockId>,
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

            block.insert_instr(effect_instr);
            // End of block instructions are jump, branch, and return
            match op {
                EffectOps::Jump | EffectOps::Branch | EffectOps::Return => {
                    close_block(function_cfg, block, id_counter, label_to_block_id);
                }

                _ => {}
            }
        }
        _ => block.insert_instr(instr),
    }
}

fn insert_code_blocks(
    function_cfg: &mut FunctionCFG,
    code: Vec<Code>,
    label_to_block_id: &mut HashMap<String, BlockId>,
) {
    let mut block = BasicBlock::default();
    let mut id_counter = 0;
    for instr in code {
        match instr {
            Code::Instruction(instr) => {
                process_instruction(
                    function_cfg,
                    &mut block,
                    instr,
                    &mut id_counter,
                    label_to_block_id,
                );
            }
            // If block hits label, then push previous block into function and set name of new one
            Code::Label { label, pos } => {
                // Cloase current block
                close_block(function_cfg, &mut block, &mut id_counter, label_to_block_id);

                // Insert new block metadata into FunctionCFG
                let name = Label {
                    name: label.clone(),
                    position: pos,
                };

                function_cfg.insert_block_id(id_counter, name);
                label_to_block_id.insert(label, id_counter);
            }
        }
    }

    // Add to block map if block has contents after parsing everything
    close_block(function_cfg, &mut block, &mut id_counter, label_to_block_id);
}

pub fn construct_cfg(program: Program) -> ProgramCFG {
    let mut program_cfg = ProgramCFG::default();
    let mut label_to_block_id = HashMap::default();
    for function in program.functions {
        let function_label = Label {
            name: function.name,
            position: function.pos,
        };

        let mut function_cfg = FunctionCFG::new(function_label);
        insert_code_blocks(&mut function_cfg, function.instrs, &mut label_to_block_id);
        function_cfg.build_sucessors(&label_to_block_id);

        program_cfg.insert_function_cfg(function_cfg);
    }

    program_cfg
}
