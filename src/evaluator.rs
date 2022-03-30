use crate::parser::Expected;
use crate::parser::AST;
use crate::Token;
use crate::TokenType;

// represents a partially evaluated expression
pub type TokenStack = Vec<Token>;

pub fn new_stack(ast: AST) -> TokenStack {
    let mut stack: TokenStack = Vec::new();
    stack.push(ast.value);
    while try_evaluate(&mut stack) {}
    match ast.left {
        Some(subtree) => stack.append(&mut new_stack(*subtree)),
        None => {}
    }
    while try_evaluate(&mut stack) {}
    match ast.middle {
        Some(subtree) => stack.append(&mut new_stack(*subtree)),
        None => {}
    }
    while try_evaluate(&mut stack) {}
    match ast.right {
        Some(subtree) => stack.append(&mut new_stack(*subtree)),
        None => {}
    }
    while try_evaluate(&mut stack) {}
    stack
}

fn evaluate(stack: &mut TokenStack) {
    let operand_two: i32 = stack[stack.len() - 1].value.parse::<i32>().unwrap(); // unwrap safe because of scanner
    let operand_one: i32 = stack[stack.len() - 2].value.parse::<i32>().unwrap(); // unwrap safe because of scanner
    let operator = stack[stack.len() - 3].clone();

    // generate operator expectors
    let plus = Expected::Value("+");
    let mul = Expected::Value("*");
    let min = Expected::Value("-");
    let div = Expected::Value("/");

    // calculate new value
    let newval = if plus.check(&operator) {
        operand_one + operand_two
    } else if min.check(&operator) {
        operand_one - operand_two
    } else if mul.check(&operator) {
        operand_one * operand_two
    } else if div.check(&operator) {
        operand_one / operand_two
    } else {
        unreachable!()
    };

    // pop last three and push new value
    stack.pop();
    stack.pop();
    stack.pop();

    stack.push(Token {
        kind: TokenType::Number,
        value: newval.to_string(),
        line: 0, // doesn't make sense here
    });
}

fn try_evaluate(stack: &mut TokenStack) -> bool {
    if stack.len() >= 3
        && stack.last().unwrap().kind == TokenType::Number
        && stack[stack.len() - 2].kind == TokenType::Number
        && stack[stack.len() - 3].kind == TokenType::Symbol
    {
        evaluate(stack);
        true
    } else {
        false
    }
}
