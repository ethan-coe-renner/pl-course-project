use crate::parser::Expected;
use crate::parser::AST;
use crate::Token;
use crate::TokenType;
use std::fmt;

pub enum EvalError {
    DivByZero {
        numerator: u32,
    },
    #[allow(dead_code)] // until 3.2
    UndeclaredVariable {
        variable: String,
    },
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

pub fn eval(ast: AST) -> Result<u32, EvalError> {
    match ast.value.kind {
        TokenType::Number => Ok(ast.value.value.parse::<u32>().unwrap()),
        TokenType::Symbol => eval_interior(
            ast.value,
            eval(*ast.left.unwrap())?,
            eval(*ast.right.unwrap())?,
        ),
        _ => unreachable!(),
    }
}

fn eval_interior(op: Token, operand_one: u32, operand_two: u32) -> Result<u32, EvalError> {
    // generate operator expectors
    if Expected::Value("+").check(&op) {
        Ok(operand_one + operand_two)
    } else if Expected::Value("-").check(&op) {
        if operand_one >= operand_two {
            // check for underflow
            Ok(operand_one - operand_two)
        } else {
            Ok(0)
        }
    } else if Expected::Value("*").check(&op) {
        Ok(operand_one * operand_two)
    } else if Expected::Value("/").check(&op) {
        match operand_two {
            0 => {
                return Err(EvalError::DivByZero {
                    numerator: operand_one,
                })
            }
            divisor => Ok(operand_one / divisor),
        }
    } else {
        // not a valid operator, scanner should not allow this
        unreachable!()
    }
}
