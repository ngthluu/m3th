pub enum Token {
    // Literal
    LitInt(i64),
    LitDouble(f64),
    LitBool(bool),
    LitStr(String),
    LitVacuum,

    // Operators
    OpAdd,
    OpMinus,
    OpMul,
    OpDiv,
    OpMod,
    OpLt,
    OpLte,
    OpGt,
    OpGte,
    OpEq,
    OpDiff,
    OpAnd,
    OpOr,
    OpNot,

    // Keywords
    KwWhen,
    KwImmut,
    KwMut,
    KwMutRef,
    KwFunction,
    KwPkg,
    KwNeed,
    KwAs,
    KwShow,

    // Others
    Comment,
    Semicolon,
    Colon,
    Return,
    Dot,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace ,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let tokens = Vec::new();
    tokens
}
