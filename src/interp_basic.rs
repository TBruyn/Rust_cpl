use expression::*;
use expression::ExprExt::*;
use expression::SExpr::*;

pub fn parse_SExpr(expression: &SExpr) -> ExprExt {
    match expression {
        SList(vec) => 
            match vec[..] {
                [ref s, ref l,  ref r] => 
                    match s {
                        SSym(sym) => return BinOpExt( sym.to_string(), 
                                                Box::new(parse_SExpr(&l)), 
                                                Box::new(parse_SExpr(&r))   ),
                        _ => panic!("Unexpected expression")
                    },

            
            
                           _ => panic!("Unexpected expression")
        }
        _ => panic!("Unexpected expression")
        // SList([SSym(s), a, b]) => BinOpExt(s, parse_SExpr(a), parse_SExpr(b))
    }
}