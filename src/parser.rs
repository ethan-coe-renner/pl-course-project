use crate::Token;
use crate::TokenType;
use std::error::Error;
use std::fmt;
use std::iter::Peekable;

pub struct AST {
    pub value: Token,
    pub left: Option<Box<AST>>,
    pub middle: Option<Box<AST>>,
    pub right: Option<Box<AST>>,
}

impl AST {
    pub fn new(value: Token) -> Self {
        AST {
            value,
            left: None,
            middle: None,
            right: None,
        }
    }
    pub fn left(mut self, node: AST) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    // will be used in 2.2
    // pub fn middle(mut self, node: AST) -> Self {
    //     self.middle = Some(Box::new(node));
    //     self
    // }
    pub fn right(mut self, node: AST) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn to_str(self, level: usize) -> String {
        let mut st = String::from("\t".repeat(level));
        st.push_str(&self.value.to_string());
        st.push('\n');
        match self.left {
            Some(subtree) => st.push_str(&subtree.to_str(level + 1)),
            None => {}
        }
        match self.middle {
            Some(subtree) => st.push_str(&subtree.to_str(level + 1)),
            None => {}
        }
        match self.right {
            Some(subtree) => st.push_str(&subtree.to_str(level + 1)),
            None => {}
        }
        st
    }
}

#[derive(Debug)]
pub enum ParseError {
    EOF,
    InvalidToken {
        found: Token,
        expected: &'static str,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EOF => write!(f, "Reached end of file without completing valid AST"),
            Self::InvalidToken { found, expected } => {
                write!(
                    f,
                    "Invalid token: found {}, expected {}",
                    found.value, expected
                )
            }
        }
    }
}

impl Error for ParseError {}

pub fn parse<I: Iterator<Item = Token>>(token_stream: &mut Peekable<I>) -> Result<AST, ParseError> {
    let tree = parse_expression(token_stream)?;

    match token_stream.next() {
        None => Ok(tree),
        Some(token) => Err(ParseError::InvalidToken {
            found: token,
            expected: "operator",
        }),
    }
}

fn parse_expression<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_term(token_stream)?;

    while peek_token_match(token_stream, "+") {
        tree = AST::new(token_stream.next().unwrap())
            .left(tree)
            .right(parse_term(token_stream)?);
    }

    Ok(tree)
}

fn parse_term<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_factor(token_stream)?;

    while peek_token_match(token_stream, "-") {
        tree = AST::new(token_stream.next().unwrap())
            .left(tree)
            .right(parse_factor(token_stream)?);
    }

    Ok(tree)
}

fn parse_factor<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_piece(token_stream)?;

    while peek_token_match(token_stream, "/") {
        tree = AST::new(token_stream.next().unwrap())
            .left(tree)
            .right(parse_piece(token_stream)?);
    }

    Ok(tree)
}

fn parse_piece<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_element(token_stream)?;

    while peek_token_match(token_stream, "*") {
        tree = AST::new(token_stream.next().unwrap())
            .left(tree)
            .right(parse_element(token_stream)?);
    }

    Ok(tree)
}

fn parse_element<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let tree: AST;
    let next_token = consume_token(token_stream)?;

    if next_token.value == *"(" {
        tree = parse_expression(token_stream)?;
        if peek_token_match(token_stream, ")") {
            consume_token(token_stream)?;
            Ok(tree)
        } else {
            match token_stream.next() {
                None => Err(ParseError::EOF),
                Some(token) => Err(ParseError::InvalidToken {
                    found: token,
                    expected: ")",
                }),
            }
        }
    } else if next_token.kind == TokenType::Number || next_token.kind == TokenType::Identifier {
        tree = AST::new(next_token);
        Ok(tree)
    } else {
        Err(ParseError::InvalidToken {
            found: next_token,
            expected: "( or number or ident",
        })
    }
}

fn consume_token<I: Iterator<Item = Token>>(token_stream: &mut I) -> Result<Token, ParseError> {
    match token_stream.next() {
        Some(token) => Ok(token),
        None => Err(ParseError::EOF),
    }
}

fn peek_token_match<I: Iterator<Item = Token>>(token_stream: &mut Peekable<I>, comp: &str) -> bool {
    match token_stream.peek() {
        Some(token) => token.value == comp,
        None => false,
    }
}
