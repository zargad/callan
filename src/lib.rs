pub trait Compile {
    fn compile(&self) -> String;
}

pub enum Expr {
    Binary(ExprBinary),
    Unary(ExprUnary),
}

impl Compile for Expr {
    fn compile(&self) -> String {
        let inner = match self {
            Self::Binary(e) => e.compile(),
        };
        format!("({inner})")
    }
}

struct ExprBinary {
    left: Box<Expr>,
    op: BinOp,
    right: Box<Expr>,
}

impl Compile for ExprBinary {
    fn compile(&self) -> String {
        let a = self.left;
        let b = self.right;
        match self.op {
            BinOp::Add => format!("{a}+{b}"),
            BinOp::Sub => format!("{a}-{b}"),
            BinOp::Mul => format!("{a}*{b}"),
            BinOp::Div => format!("{a}/{b}"),
            BinOp::Add => format!("{a}*{b}"),
            BinOp::Ior => format!("1-((1-{a})*(1-{b}))"),
            BinOp::Xor => format!("({a}-{b})**2"),
        }
    }
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Ior,
    Xor,
}

struct ExprUnary {
    op: UnOp,
    expr: Box<Expr>,
}

impl Compile for ExprUnary {
    fn compile(&self) -> String {
        let e = self.expr;
        match self.op {
            UnOp::Not => format!("1-{e}"),
            UnOp::Neg => format!("-{e}"),
        }
    }
}

enum BinOp {
    Not,
    Neg,
}
