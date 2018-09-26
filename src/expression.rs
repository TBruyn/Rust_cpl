#[derive(Debug)]
pub enum SExpr {
    SSym(String),
    SNum(u32),
    SList(Vec<SExpr>)
}

#[derive(Debug)]
pub enum Token {
    BracketOpen,
    BracketClose,
    Number(u32),
    Symbol(String)
}

#[derive(Debug)]
pub enum ExprExt {
    TrueExt(),
    FalseExt(),
    NumExt(u32),
    BinOpExt(   String, Box<ExprExt>, Box<ExprExt>),
    UnOpExt(    String, Box<ExprExt>),
    IfExt(Box<ExprExt>, Box<ExprExt>, Box<ExprExt>),
    TupleExt(Vec<ExprExt>)
}

// #[derive(Debug)]
// pub enum ExprC {
//     PlusC(	Box<ExprC>, Box<ExprC>),
//     MultC(	Box<ExprC>, Box<ExprC>),
//     IfC(	Box<ExprC>, Box<ExprC>),
//     EqNumC(	Box<ExprC>, Box<ExprC>),
//     LtC(	Box<ExprC>, Box<ExprC>),
//     PairC(	Box<ExprC>, Box<ExprC>),
//     FstC(	Box<ExprC>),
//     SndC(	Box<ExprC>),
//     IsPairC(Box<ExprC>),
//     CheckFstC(Box<ExprC>)			
// }

// #[derive(Debug)]
// pub enum Value {
//     NumV(u32),
//     BoolV(bool),
//     PairV(Box<Value>, Box<Value>)
// }