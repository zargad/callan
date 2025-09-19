pub enum Expr {
    Binary(ExprBinary),
    Unary(ExprUnary),
    Lit(ExprLit),
    If(ExprIf),
    Var(ExprVar),
}

pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
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
    pub op: UnOp,
    pub expr: Box<Expr>,
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
    pub cond: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

pub struct ExprVar {
    pub name: char,
}
