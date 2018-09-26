#![feature(slice_patterns)]

mod expression;
mod tokenizer;
mod interp_basic;

fn main() {
    let test_input = String::from("(+ 1 (- (- 1)  3))");
    let _tokens = tokenizer::string_to_tokens(&test_input);
    println!("{:?}", tokenizer::parse_string(&test_input));;
}