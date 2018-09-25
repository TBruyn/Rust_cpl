mod tokenizer;

fn main() {
    let test_input = String::from("(+ 1 (- (- 1)  3))");
    let tokens = tokenizer::string_to_tokens(&test_input);
    let ast = tokenizer::tokens_to_expression_tree(tokens);
    println!("{:?}", tokenizer::parse_string(&test_input));;
}