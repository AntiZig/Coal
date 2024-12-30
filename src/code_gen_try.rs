// this is specifically code gen for x86_64, done naively.

enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RBP,
    RSP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

enum Instruction {
    Add(Register, Register),
    AddImm(Register, i64),
    Sub(Register, Register),
    SubImm(Register, i64),
    Push(Register),
}

enum Expr {
    Addition,
    Subtraction,
    Multiplication,
    Number(i64),
}

struct ASTNode {
    head: Expr,
    nodes: Vec<ASTNode>,
}

impl ASTNode {}
