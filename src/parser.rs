use crate::Token;
use crate::TokenType;
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
    pub fn middle(mut self, node: AST) -> Self {
        self.middle = Some(Box::new(node));
        self
    }
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

pub fn parse<I: Iterator<Item = Token>>(token_stream: &mut Peekable<I>) -> Result<AST, ParseError> {
    let tree = parse_statement(token_stream)?;

    match token_stream.next() {
        None => Ok(tree),
        Some(token) => Err(ParseError::InvalidToken {
            found: token,
            expected: "operator",
        }),
    }
}

fn parse_statement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_basestatement(token_stream)?;
    while peek_token_match(token_stream, ";") {
        tree = AST::new(token_stream.next().unwrap())
            .left(tree)
            .right(parse_basestatement(token_stream)?);
    }
    Ok(tree)
}

fn parse_while_statement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree = AST::new(expect(token_stream, "while")?).left(parse_expression(token_stream)?);
    expect(token_stream, "do")?;

    tree = tree.right(parse_statement(token_stream)?);

    expect(token_stream, "endwhile")?;

    Ok(tree)
}

fn parse_if_statement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    Ok(AST::new(expect(token_stream, "if")?)
        .left(parse_expression(token_stream)?)
        .middle(parse_statement(token_stream)?)
        .right(parse_statement(token_stream)?))
}

fn parse_assignment<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let tree = parse_element(token_stream)?;
    Ok(AST::new(expect(token_stream, ":=")?)
        .left(tree)
        .right(parse_expression(token_stream)?))
}

fn parse_basestatement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    //basestatement ::= assignment | ifstatement | whilestatement | skip
    if peek_token_match(token_stream, "while") {
        parse_while_statement(token_stream)
    } else if peek_token_match(token_stream, "if") {
        parse_if_statement(token_stream)
    } else if peek_token_match(token_stream, "skip") {
        Ok(AST::new(token_stream.next().unwrap()))
    } else {
        parse_assignment(token_stream)
    }
}

// TODO: refactor code to use this function, probably parse_element and maybe others
fn expect<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
    expected: &'static str,
) -> Result<Token, ParseError> {
    let next_token = consume_token(token_stream)?;

    if next_token.value == *expected {
        Ok(next_token)
    } else {
        Err(ParseError::InvalidToken {
            found: next_token,
            expected,
        })
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
