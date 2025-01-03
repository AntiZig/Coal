use crate::ir::ir_types::{IRBlock, IROp, IRValue, IReg, LocalCtx};
use std::collections::HashMap;

fn reg_to_str_map() -> HashMap<Register, String> {
    HashMap::from([
        (Register::RAX, "rax".to_string()),
        (Register::RBX, "rbx".to_string()),
        (Register::RCX, "rcx".to_string()),
        (Register::RDX, "rdx".to_string()),
        (Register::RDI, "rdi".to_string()),
        (Register::RSI, "rsi".to_string()),
        (Register::R8, "r8".to_string()),
        (Register::R9, "r9".to_string()),
        (Register::R10, "r10".to_string()),
        (Register::R11, "r11".to_string()),
        (Register::R12, "r12".to_string()),
        (Register::R13, "r13".to_string()),
        (Register::R14, "r14".to_string()),
        (Register::R15, "r15".to_string()),
    ])
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Register {
    RAX = 0,
    RBX = 1,
    RCX = 2,
    RDX = 3,
    RDI = 4,
    RSI = 5,
    R8 = 6,
    R9 = 7,
    R10 = 8,
    R11 = 9,
    R12 = 10,
    R13 = 11,
    R14 = 12,
    R15 = 13,
}

fn get_starting_scratches() -> Vec<Register> {
    vec![Register::RAX, Register::RDX, Register::RDI]
}

fn get_starter_free() -> Vec<Register> {
    vec![
        Register::RBX,
        Register::RCX,
        Register::RSI,
        Register::R8,
        Register::R9,
        Register::R10,
        Register::R11,
        Register::R12,
        Register::R14,
        Register::R15,
    ]
}

fn allocate_scratch(scratches: &mut Vec<Register>) -> Option<Register> {
    scratches.pop()
}

fn get_reg(scratches: &mut Vec<Register>, stack: HashMap<IReg, u32>) -> Register {
    match allocate_scratch(scratches) {
        Some(reg) => reg,
        _ => unimplemented!(),
    }
}

fn free_scratch(reg: Register, scratches: &mut Vec<Register>) {
    scratches.push(reg);
}

#[derive(Debug)]
enum Memory {
    Register(Register),
    Stack(usize),
}

pub fn top_down_init(block: IRBlock) {
    let ops = block.ctx.ops;
    let mut prios: Vec<(usize, usize)> = (0..ops.len()).map(|i| (0, i)).collect();
    for op in ops.iter() {
        match op {
            IROp::Load(_) => continue,
            IROp::Mov(reg, _) => prios[*reg].0 += 1,
            IROp::Store(reg, _) => prios[*reg].0 += 1,
            IROp::Add(r1, r2) => {
                prios[*r1].0 += 1;
                prios[*r2].0 += 1;
            }
            IROp::Sub(r1, r2) => {
                prios[*r1].0 += 1;
                prios[*r2].0 += 1;
            }
            IROp::Mul(r1, r2) => {
                prios[*r1].0 += 1;
                prios[*r2].0 += 1;
            }
            IROp::Call(_, regs) => panic!("calls are not yet implemented."), //regs.iter().map(|r| prios[*r].0 += 1).collect(),
        }
    }

    prios.sort_by(|(occ1, _), (occ2, _)| occ1.partial_cmp(occ2).unwrap());

    let mut map: HashMap<IReg, Memory> = HashMap::new();

    let free_regs = get_starter_free();
    let mut scratches = get_starting_scratches();
    println!("{:?}", prios);
    for reg in free_regs.into_iter() {
        let prio = prios.pop();
        match prio {
            Some(p) => {
                map.insert(p.1, Memory::Register(reg));
            }
            None => scratches.push(reg),
        };
    }
    let stack_size = prios.len();
    let stack_opts: Vec<_> = prios
        .iter()
        .enumerate()
        .map(|(i, (_, vr))| map.insert(*vr, Memory::Stack(i)))
        .collect();
    println!("{:?}", map);

    unimplemented!()
}
