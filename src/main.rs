mod expression;
mod tokenizer;
mod interp_basic;

fn main() {
    let test_input_1 = String::from("(+ 1 (- (- 1)  3))");
    let test_input_2  = String::from("(if true 1 (+ 1 (- 2)))");

    let expr_ext_1 = interp_basic::parse_s_expr(&tokenizer::parse_string(&test_input_1));
    let expr_ext_2 = interp_basic::parse_s_expr(&tokenizer::parse_string(&test_input_2));

    println!("1. {:?}", expr_ext_1);
    println!("2. {:?}", expr_ext_2);

}