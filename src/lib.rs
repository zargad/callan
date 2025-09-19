pub trait Compile {
    fn compile(&self) -> String;
}

pub enum Expr {
    Binary(ExprBinary),
    Unary(ExprUnary),
    Lit(ExprLit),
    If(ExprIf),
}

impl Compile for Expr {
    fn compile(&self) -> String {
        let inner = match self {
            Self::Binary(e) => e.compile(),
            Self::Unary(e) => e.compile(),
            Self::Lit(e) => e.compile(),
            Self::If(e) => e.compile(),
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
        let a = self.left.compile();
        let b = self.right.compile();
        match self.op {
            BinOp::Add => format!("{a}+{b}"),
            BinOp::Sub => format!("{a}-{b}"),
            BinOp::Mul => format!("{a}*{b}"),
            BinOp::Div => format!("{a}/{b}"),
            BinOp::And => format!("{a}*{b}"),
            BinOp::Or => format!("1-((1-{a})*(1-{b}))"),
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
    Or,
    Xor,
}

struct ExprUnary {
    op: UnOp,
    expr: Box<Expr>,
}

impl Compile for ExprUnary {
    fn compile(&self) -> String {
        let e = self.expr.compile();
        match self.op {
            UnOp::Not => format!("1-{e}"),
            UnOp::Neg => format!("-{e}"),
        }
    }
}

enum UnOp {
    Not,
    Neg,
}

enum ExprLit {
    Int(String),
    Float(String),
    Bool(bool),
}

impl Compile for ExprLit {
    fn compile(&self) -> String {
        match self {
            Self::Int(s) => format!("{s}"),
            Self::Float(s) => format!("{s}"),
            Self::Bool(b) => if *b {
                format!("1")
            } else {
                format!("0")
            },
        }
    }
}

struct ExprIf {
    cond: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
}

impl Compile for ExprIf {
    fn compile(&self) -> String {
        format!(
            "({COND}*{THEN})+((1-{COND})*{ELSE})",
            COND = self.cond.compile(),
            THEN = self.then_branch.compile(),
            ELSE = self.else_branch.compile(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_compile() {
        let expr = Expr::Binary(
            ExprBinary {
                left: Box::new(Expr::Lit(ExprLit::Bool(false))),
                op: BinOp::And,
                right: Box::new(Expr::Unary(ExprUnary {
                    op: UnOp::Not,
                    expr: Box::new(Expr::Lit(ExprLit::Float("3.14".to_string()))),
                })),
            }
        );
        let result = expr.compile();
        assert_eq!(&result, "((0)*(1-(3.14)))");
    }
}
