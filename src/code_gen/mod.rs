pub mod instructions;
pub mod local_alloc;

use std::collections::HashMap;

use crate::code_gen::local_alloc::{Memory, Register};
use crate::ir::ir_types::{IRBlock, IROp, IRValue, IReg};

pub struct Ctx {
    pub rtol: HashMap<IReg, Memory>,
    pub scratches: Vec<Register>,
    pub stack_size: usize,
}
