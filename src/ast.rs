use crate::token::Token;

type AST = Vec<Expr>;

enum Expr {
    Int(i64),
    Double(f64),
    Bool(bool),
    Str(String),
    UnaryOp {
        value: Box<Expr>,
        op: Token,
    },
    BinaryOp {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Token,
    },
    FuncDecl {
        name: String,
        args: Vec<Expr>,
        body: Vec<Expr>,
    },

}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
}
