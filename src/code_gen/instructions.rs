use crate::code_gen::Ctx;
use crate::code_gen::Register;
use crate::ir::ir_types::{IRBlock, IROp, IRValue, IReg};

use super::local_alloc::Memory;

#[derive(Debug)]
pub enum Instr {
    Push(Register),
    Add(Register, Register),
    AddImm(Register, i64),
    Pop(Register),
    Mov(Memory, Memory),
    MovImm(Memory, u64),
}

fn alloc_scratch(scratches: &mut Vec<Register>) -> Register {
    match scratches.pop() {
        Some(reg) => reg,
        None => panic!("not enough scratch registers"),
    }
}

fn free_scratch(reg: Register, scratches: &mut Vec<Register>) {
    scratches.push(reg);
}

pub fn block_to_asm(block: IRBlock, ctx: &mut Ctx) -> Vec<Instr> {
    let mut instrs: Vec<Instr> = Vec::new();
    if ctx.stack_size > 0 {
        instrs.push(Instr::AddImm(
            Register::RSP,
            ((ctx.stack_size * 8) as i64) * -1,
        ));
    }
    block
        .ctx
        .ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| op_to_asm(i, op, &mut instrs, ctx))
        .collect::<Vec<_>>();
    instrs
}

fn op_to_asm(vreg: IReg, op: IROp, instrs: &mut Vec<Instr>, ctx: &mut Ctx) {
    match op {
        IROp::Load(val) => load_to_asm(vreg, val, instrs, ctx),
        IROp::Store(giver, _) => mov_to_asm(vreg, giver, instrs, ctx),
        IROp::Mov(giver, _) => mov_to_asm(vreg, giver, instrs, ctx),
        IROp::Add(r1, r2) => add_to_asm(vreg, r1, r2, instrs, ctx),
        _ => unimplemented!(),
    }
}

fn add_to_asm(receiver: IReg, vr1: IReg, vr2: IReg, instrs: &mut Vec<Instr>, ctx: &mut Ctx) {
    let r1 = get_to_reg(vr1, instrs, ctx);
    let r2 = get_to_reg(vr2, instrs, ctx);
    instrs.push(Instr::Add(r1.clone(), r2.clone()));
    get_out_of_reg(vr2, r2, instrs, ctx);
    let r3 = get_to_reg(receiver, instrs, ctx);
    instrs.push(Instr::Mov(
        Memory::Register(r3.clone()),
        Memory::Register(r1.clone()),
    ));
    get_out_of_reg(receiver, r3, instrs, ctx);
    get_out_of_reg(vr1, r1, instrs, ctx);
}

fn mov_to_asm(receiver: IReg, giver: IReg, instrs: &mut Vec<Instr>, ctx: &mut Ctx) {
    let r1 = get_to_reg(receiver, instrs, ctx);
    let r2 = get_to_reg(giver, instrs, ctx);

    instrs.push(Instr::Mov(
        Memory::Register(r1.clone()),
        Memory::Register(r2.clone()),
    ));

    get_out_of_reg(receiver, r1, instrs, ctx);
    get_out_of_reg(giver, r2, instrs, ctx);
}

fn get_to_reg(reg: IReg, instrs: &mut Vec<Instr>, ctx: &mut Ctx) -> Register {
    let mem = ctx.rtol.get(&reg).unwrap();
    match mem {
        Memory::Stack(place) => {
            let r1 = alloc_scratch(&mut ctx.scratches);
            instrs.push(Instr::Mov(Memory::Register(r1.clone()), mem.clone()));
            r1
        }
        Memory::Register(reg) => reg.clone(),
    }
}

fn get_out_of_reg(vreg: IReg, register: Register, instrs: &mut Vec<Instr>, ctx: &mut Ctx) {
    let mem = ctx.rtol.get(&vreg).unwrap();
    match mem {
        Memory::Stack(place) => {
            instrs.push(Instr::Mov(mem.clone(), Memory::Register(register.clone())));
            free_scratch(register, &mut ctx.scratches);
        }
        Memory::Register(_) => (),
    };
}

fn load_to_asm(reg: IReg, val: u64, instrs: &mut Vec<Instr>, ctx: &mut Ctx) {
    let r1 = get_to_reg(reg, instrs, ctx);
    instrs.push(load_to_mem(val, Memory::Register(r1.clone())));
    get_out_of_reg(reg, r1, instrs, ctx);
}

fn load_to_mem(val: u64, mem: Memory) -> Instr {
    Instr::MovImm(mem, val)
}
