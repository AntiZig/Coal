use crate::ir::Type;
use std::collections::HashMap;

fn reg_to_str(reg: IReg) -> String {
    "r".to_string() + &reg.to_string()
}

// global context of all blocks.
#[derive(Debug)]
pub struct GlobalCtx {
    labels: HashMap<String, IRBlock>,
}

// local context of the block.
#[derive(Debug)]
pub struct LocalCtx {
    pub ops: Vec<IROp>,
    pub vtoi: HashMap<String, usize>,
    pub lcounter: usize,
}

type Label = String;

#[derive(Debug)]
pub struct IRBlock {
    pub label: Label,
    pub params: Vec<Type>,
    pub ctx: LocalCtx,
}

impl ToString for IRBlock {
    fn to_string(&self) -> String {
        let name = &self.label;

        let header = String::from("block ") + &name + &String::from(" ") + &String::from("\n");
        let code = self
            .ctx
            .ops
            .clone()
            .into_iter()
            .enumerate()
            .fold("".to_string(), |acc, (i, op)| {
                acc + &op.to_string_triple(i) + &String::from("\n")
            });
        header + &code
    }
}

type MetaData = String;
pub type IReg = usize;

#[derive(Debug, Clone)]
pub enum IROp {
    Load(IRValue),
    Mov(IReg, MetaData),
    Store(IReg, MetaData),
    Add(IReg, IReg),
    Sub(IReg, IReg),
    Mul(IReg, IReg),
    Call(Label, Vec<IReg>),
}

impl IROp {
    fn to_string_triple(&self, i: usize) -> String {
        match self {
            Self::Mov(reg, meta) => {
                String::from("mov ")
                    + &reg_to_str(i)
                    + &String::from(", ")
                    + &reg_to_str(*reg)
                    + &String::from(" //")
                    + meta
            }
            Self::Store(reg, meta) => {
                String::from("str ")
                    + &reg_to_str(i)
                    + &String::from(", ")
                    + &reg_to_str(*reg)
                    + &String::from(" //")
                    + meta
            }
            Self::Add(r1, r2) => {
                String::from("add ")
                    + &reg_to_str(i)
                    + &String::from(", ")
                    + &reg_to_str(*r1)
                    + &String::from(", ")
                    + &reg_to_str(*r2)
            }
            Self::Sub(r1, r2) => {
                String::from("sub ")
                    + &reg_to_str(i)
                    + &String::from(", ")
                    + &reg_to_str(*r1)
                    + &String::from(", ")
                    + &reg_to_str(*r2)
            }
            Self::Mul(r1, r2) => {
                String::from("mul ")
                    + &reg_to_str(i)
                    + &String::from(", ")
                    + &reg_to_str(*r1)
                    + &String::from(", ")
                    + &reg_to_str(*r2)
            }
            Self::Load(val) => {
                String::from("ld ") + &reg_to_str(i) + &String::from(", ") + &val.to_string()
            }
            Self::Call(label, regs) => {
                String::from("call ")
                    + label
                    + &String::from("(")
                    + &regs
                        .into_iter()
                        .map(|&reg| reg_to_str(reg))
                        .collect::<Vec<_>>()
                        .join(", ")
                    + &String::from(")")
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRValue {
    Reg(IReg),
    U64(u64),
    Var(String),
}

impl ToString for IRValue {
    fn to_string(&self) -> String {
        match self {
            IRValue::Reg(register) => reg_to_str(*register),
            IRValue::U64(n) => n.to_string(),
            IRValue::Var(str) => str.to_string(),
        }
    }
}
