pub enum Expr {
    Binary(ExprBinary),
    Unary(ExprUnary),
    Lit(ExprLit),
    If(ExprIf),
    Var(ExprVar),
}

pub struct ExprBinary {
    left: Box<Expr>,
    op: BinOp,
    right: Box<Expr>,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
}

pub struct ExprUnary {
    op: UnOp,
    expr: Box<Expr>,
}

pub enum UnOp {
    Not,
    Neg,
}

pub enum ExprLit {
    Number(String),
    Bool(bool),
}

pub struct ExprIf {
    cond: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
}

pub struct ExprVar {
    name: char,
}
