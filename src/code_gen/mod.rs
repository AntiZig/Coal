pub mod local_alloc;

use std::collections::HashMap;

use crate::code_gen::local_alloc::Register;
use crate::ir::ir_types::{IRBlock, IROp, IRValue, IReg};

enum Instr {
    Push(Register),
    Add(Register, Register),
    Pop(Register),
    Load(u64),
    Mov(Register, Register),
}

fn block_to_instructions(block: IRBlock) {
    unimplemented!()
}

fn op_to_instr(op: IROp) {
    unimplemented!()
}

fn add_to_instr(stack: Vec<u32>) {
    // mov rax, r1
    // mov r10, r2
    // add rax, r10
    // push rax
    //
}
