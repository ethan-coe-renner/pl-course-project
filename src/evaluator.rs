use crate::parser::Expected;
use crate::parser::AST;
use crate::Token;
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
    if ast.left.is_none() && ast.right.is_none() {
        // ast is a leaf node
        Ok(ast.value.value.parse::<u32>().unwrap())
    } else {
        // ast is an internal node or root
        eval_op(
            ast.value,
            eval(*ast.left.unwrap())?,
            eval(*ast.right.unwrap())?,
        )
    }
}

fn eval_op(op: Token, operand_one: u32, operand_two: u32) -> Result<u32, EvalError> {
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
