use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Pointer},
};

#[derive(Debug)]
enum Expr {
    Num(u64),
    Name(String),
    Addition(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    FuncCall(String, Vec<Expr>),
}

#[derive(Debug)]
enum Stmt {
    Assign(String, Expr),
    Declare(String, Expr),
    For(Expr, Vec<Stmt>),
    If(Expr, Vec<Stmt>),
}

type Type = String;

#[derive(Debug)]
struct Fn {
    name: String,
    ttype: Type,
    params: Vec<(String, Type)>,
    body: Vec<Stmt>,
}

#[derive(Debug)]
struct IRBlock {
    name: String,
    ttype: Type,
    params: Vec<Type>,
    ctx: IRctx,
}

impl ToString for IRBlock {
    fn to_string(&self) -> String {
        let name = &self.name;
        let t = &self.ttype;

        let header = String::from("block ") + &name + &String::from(" ") + &t + &String::from("\n");
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

type IReg = usize;

#[derive(Debug, Clone)]
enum IROp {
    Load(IRValue),
    Mov(IReg),
    Store(IReg),
    Add(IReg, IReg),
    Sub(IReg, IReg),
    Mul(IReg, IReg),
}

impl IROp {
    fn to_string_triple(&self, i: usize) -> String {
        match self {
            Self::Mov(reg) => {
                String::from("mov ") + &reg_to_str(i) + &String::from(", ") + &reg_to_str(*reg)
            }
            Self::Store(reg) => {
                String::from("str ") + &reg_to_str(i) + &String::from(", ") + &reg_to_str(*reg)
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
            _ => unimplemented!(),
        }
    }
}

impl ToString for IROp {
    fn to_string(&self) -> String {
        match self {
            Self::Mov(reg) => String::from("mov ") + &reg_to_str(*reg),
            Self::Store(reg) => String::from("str ") + &reg_to_str(*reg),
            Self::Add(r1, r2) => {
                String::from("add ") + &reg_to_str(*r1) + &String::from(" ") + &reg_to_str(*r2)
            }
            Self::Sub(r1, r2) => {
                String::from("sub ") + &reg_to_str(*r1) + &String::from(" ") + &reg_to_str(*r2)
            }
            Self::Mul(r1, r2) => {
                String::from("mul ") + &reg_to_str(*r1) + &String::from(" ") + &reg_to_str(*r2)
            }
            Self::Load(val) => String::from("ld ") + &val.to_string(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
enum IRValue {
    Reg(IReg),
    U64(u64),
    Var(String),
}

fn reg_to_str(reg: IReg) -> String {
    "r".to_string() + &reg.to_string()
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

fn fn_to_ir(func: Fn) -> IRBlock {
    let mut block = IRBlock {
        name: func.name,
        ttype: func.ttype,
        params: func
            .params
            .iter()
            .map(|(name, ttype)| ttype.clone())
            .collect(),
        ctx: IRctx {
            ops: Vec::new(),
            vtoi: HashMap::new(),
        },
    };
    func.body
        .into_iter()
        .map(|stmt| stmt_to_ir(stmt, &mut block.ctx))
        .collect::<Vec<_>>();
    block
}

fn stmt_to_ir(stmt: Stmt, ctx: &mut IRctx) -> IReg {
    match stmt {
        Stmt::Assign(var, e) => assign_to_ir(var, e, ctx),
        Stmt::Declare(var, e) => declare_to_ir(var, e, ctx),
        Stmt::For(cond, stmts) => unimplemented!(),
        Stmt::If(cond, stmts) => unimplemented!(),
    }
}

fn declare_to_ir(var: String, expr: Expr, ctx: &mut IRctx) -> IReg {
    let r1 = expr_to_ir(expr, ctx);
    let op = IROp::Store(r1);
    ctx.ops.push(op);
    let reg = ctx.ops.len() - 1;
    // TODO: name collisions, local name for each block etc.
    let _ = ctx.vtoi.insert(var, reg);
    reg
}

fn assign_to_ir(var: String, expr: Expr, ctx: &mut IRctx) -> IReg {
    let r1 = expr_to_ir(expr, ctx);
    let op = IROp::Store(r1);
    ctx.ops.push(op);
    let reg = ctx.ops.len() - 1;
    // TODO: check if there is no variable by that name yet, crash.
    // for now assume correct inputs.
    ctx.vtoi.entry(var).and_modify(|v| *v = reg).or_insert(reg);
    reg
}

// TODO: refactor this function
fn expr_to_ir(e: Expr, ctx: &mut IRctx) -> IReg {
    match e {
        Expr::Addition(e1, e2) => add_to_ir(e1, e2, ctx),
        Expr::Sub(e1, e2) => sub_to_ir(e1, e2, ctx),
        Expr::Mul(e1, e2) => mul_to_ir(e1, e2, ctx),
        Expr::Num(n) => num_to_ir(n, ctx),
        Expr::Name(str) => var_to_ir(str, ctx),
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
struct IRctx {
    ops: Vec<IROp>,
    vtoi: HashMap<String, usize>,
}

// TODO: generalize next 3 functions
fn mul_to_ir(e1: Box<Expr>, e2: Box<Expr>, ctx: &mut IRctx) -> IReg {
    let r1 = expr_to_ir(*e1, ctx);
    let r2 = expr_to_ir(*e2, ctx);
    let mul = IROp::Mul(r1, r2);
    ctx.ops.push(mul);
    ctx.ops.len() - 1
}

fn sub_to_ir(e1: Box<Expr>, e2: Box<Expr>, ctx: &mut IRctx) -> IReg {
    let r1 = expr_to_ir(*e1, ctx);
    let r2 = expr_to_ir(*e2, ctx);
    let sub = IROp::Sub(r1, r2);
    ctx.ops.push(sub);
    ctx.ops.len() - 1
}

fn add_to_ir(e1: Box<Expr>, e2: Box<Expr>, ctx: &mut IRctx) -> IReg {
    let r1 = expr_to_ir(*e1, ctx);
    let r2 = expr_to_ir(*e2, ctx);
    let add = IROp::Add(r1, r2);
    ctx.ops.push(add);
    ctx.ops.len() - 1
}

fn num_to_ir(num: u64, ctx: &mut IRctx) -> IReg {
    let val = IRValue::U64(num);
    let op = IROp::Load(val);
    ctx.ops.push(op);
    ctx.ops.len() - 1
}

fn var_to_ir(var: String, ctx: &mut IRctx) -> IReg {
    let index = match ctx.vtoi.get(&var) {
        Some(k) => k,
        None => panic!("use of variable before declaration."),
    };

    let mov = IROp::Mov(*index);
    ctx.ops.push(mov);
    ctx.ops.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_ast() -> Fn {
        let expr1 = Expr::Sub(
            Box::new(Expr::Addition(
                Box::new(Expr::Num(32)),
                Box::new(Expr::Num(49)),
            )),
            Box::new(Expr::Num(32)),
        );
        let stmt1 = Stmt::Declare("a".to_string(), expr1);
        let expr2 = Expr::Sub(
            Box::new(Expr::Addition(
                Box::new(Expr::Num(32)),
                Box::new(Expr::Name(String::from("a"))),
            )),
            Box::new(Expr::Num(32)),
        );
        let stmt2 = Stmt::Assign("a".to_string(), expr2);
        Fn {
            name: String::from("test function"),
            ttype: String::from("kek"),
            params: Vec::from([(String::from("lol"), String::from("kekington"))]),
            body: Vec::from([stmt1, stmt2]),
        }
    }

    #[test]
    fn printing_test() {
        let func = get_ast();
        let ret = fn_to_ir(func);
        println!("{:?}", ret);
        println!("{}", ret.to_string());
    }
}
