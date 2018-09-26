use expression::SExpr;
use expression::Token;

pub fn parse_string(input: &String) -> SExpr {
    return tokens_to_expression_tree(string_to_tokens(input));
}

pub fn string_to_tokens(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut symbol_stack: Vec<char> = Vec::new();
    let mut number_stack: Vec<char> = Vec::new();

    for c in input.chars() {
        match c {
            '(' => {
                        flush(&mut tokens, &mut symbol_stack, &mut number_stack);
                        tokens.push(Token::BracketOpen);
                    },
            ')' => {
                        flush(&mut tokens, &mut symbol_stack, &mut number_stack);
                        tokens.push(Token::BracketClose);
                    },
            ' ' =>      flush(&mut tokens, &mut symbol_stack, &mut number_stack),
            _   => {
                        if c >= '0' && c <= '9' && symbol_stack.is_empty() {
                            number_stack.push(c);
                        } else {
                            symbol_stack.push(c);
                        }
                    },
        }
    }

    return tokens;
}

pub fn tokens_to_expression_tree(mut input: Vec<Token>) -> SExpr {
    input.remove(0);
    create_expression(&mut input)
}

fn create_expression(input: &mut Vec<Token>) -> SExpr {

    let mut expr_list: Vec<SExpr> = Vec::new();

    let mut current;
    while !input.is_empty() {
        current = input.remove(0);
        match current {
            Token::BracketClose => return SExpr::SList(expr_list),
            Token::BracketOpen  => expr_list.push(create_expression(input)),
            Token::Number(n)    => expr_list.push(SExpr::SNum(n)),
            Token::Symbol(s)    => expr_list.push(SExpr::SSym(s))
        }
    }

    return SExpr::SList(Vec::new());
}

fn flush(tokens: &mut Vec<Token>, symbol_stack: &mut Vec<char>, number_stack: &mut Vec<char>) {
    if !symbol_stack.is_empty() { 
        flush_symbol(symbol_stack, tokens);
    } else if !number_stack.is_empty() {
        flush_number(number_stack, tokens);
    }
}

fn flush_number(number_stack: &mut Vec<char>, tokens: &mut Vec<Token>) {
    let number_string: String = number_stack.iter().collect();
    tokens.push(Token::Number(number_string.parse::<u32>().unwrap()));
    number_stack.clear();
}

fn flush_symbol(symbol_stack: &mut Vec<char>, tokens: &mut Vec<Token>) {
    let symbol_string: String = symbol_stack.iter().collect();
    tokens.push(Token::Symbol(symbol_string));
    symbol_stack.clear();
}