pub trait Compile {
    fn compile(&self) -> String;
}

pub enum Expr {
    Binary(ExprBinary),
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
        }
    }
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
