use crate::parser::Expected;
use crate::parser::AST;
use crate::Token;
use crate::TokenType;
use std::fmt;

// represents a partially evaluated expression
pub type TokenStack = Vec<Token>;

pub enum EvalError {
    DivByZero { numerator: u32 },
    UndeclaredVariable { variable: String },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DivByZero { numerator } => write!(f, "divide by zero at {} / 0", numerator),
            Self::UndeclaredVariable { variable } => {
                write!(f, "use of undeclared variable \"{}\"", variable)
            }
        }
    }
}

pub fn new_stack(ast: AST) -> Result<TokenStack, EvalError> {
    let mut stack: TokenStack = Vec::new();
    stack.push(ast.value);
    while try_evaluate(&mut stack)? {}
    match ast.left {
        Some(subtree) => stack.append(&mut new_stack(*subtree)?),
        None => {}
    }
    while try_evaluate(&mut stack)? {}
    match ast.middle {
        Some(subtree) => stack.append(&mut new_stack(*subtree)?),
        None => {}
    }
    while try_evaluate(&mut stack)? {}
    match ast.right {
        Some(subtree) => stack.append(&mut new_stack(*subtree)?),
        None => {}
    }
    while try_evaluate(&mut stack)? {}
    Ok(stack)
}

fn evaluate(stack: &mut TokenStack) -> Result<(), EvalError> {
    let operand_two: u32 = stack[stack.len() - 1].value.parse::<u32>().unwrap(); // unwrap safe because of scanner
    let operand_one: u32 = stack[stack.len() - 2].value.parse::<u32>().unwrap(); // unwrap safe because of scanner
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
        if operand_one >= operand_two {
            // need to check for underflow
            operand_one - operand_two
        } else {
            0
        }
    } else if mul.check(&operator) {
        operand_one * operand_two
    } else if div.check(&operator) {
        match operand_two {
            0 => {
                return Err(EvalError::DivByZero {
                    numerator: operand_one,
                })
            }
            divisor => operand_one / divisor,
        }
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
    Ok(())
}

fn try_evaluate(stack: &mut TokenStack) -> Result<bool, EvalError> {
    if stack.len() >= 3
        && stack.last().unwrap().kind == TokenType::Number
        && stack[stack.len() - 2].kind == TokenType::Number
        && stack[stack.len() - 3].kind == TokenType::Symbol
    {
        evaluate(stack)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
