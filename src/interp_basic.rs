use expression::*;
use expression::ExprExt::*;
use expression::SExpr::*;

pub fn parse_s_expr(expression: &SExpr) -> ExprExt {
    match expression {
        SList(vec) => 
            match vec[..] {
                [ref s, ref l,  ref r] => 
                    match s {
                        SSym(sym) => return BinOpExt( sym.to_string(), 
                                                Box::new(parse_s_expr(&l)), 
                                                Box::new(parse_s_expr(&r))   ),
                        _ => panic!("Unexpected expression: {:?}", s)
                    },
                [ref s, ref e] => 
                    match s {
                        SSym(sym) => return UnOpExt( sym.to_string(), 
                                                Box::new(parse_s_expr(&e))  ),
                        _ => panic!("Unexpected expression: {:?}", s)
                    },
                [ref s, ref c, ref l, ref r] => 
                    match s {
                        SSym(sym) if sym == "if" => return IfExt( 
                                                Box::new(parse_s_expr(&c)),
                                                Box::new(parse_s_expr(&l)),
                                                Box::new(parse_s_expr(&r))  ),
                        _ => panic!("Unexpected expression: {:?}", s)
                    },
                _ => {
                        match vec[0] {
                            SSym(ref s) => match s.as_ref() {
                                "tuple" => {
                                    let mut items: Vec<ExprExt> = Vec::new();
                                    for ref item in &vec[1..] {
                                        items.push(parse_s_expr(&item));
                                    }
                                    return TupleExt(items);
                                },
                                _ => panic!("Unexpected expression: {:?}", vec)
                            },
                            _ => panic!("Unexpected expression: {:?}", vec)
                        }
                    }
            },
        SSym(ref s) =>
            match s.as_ref() {
                "true" => TrueExt(),
                "false"=> FalseExt(),
                _ => panic!("Unexpected expression")
            },
        SNum(ref n) => NumExt(*n)
    }
}

// #[derive(Debug)]
// pub enum ExprExt {
//     TrueExt(),
//     FalseExt(),
//     NumExt(u32),
//     BinOpExt(   String, Box<ExprExt>, Box<ExprExt>),
//     UnOpExt(    String, Box<ExprExt>),
//     IfExt(Box<ExprExt>, Box<ExprExt>, Box<ExprExt>),
//     TupleExt(Vec<ExprExt>)
// }