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

pub enum Expected {
    Type(TokenType),
    Value(&'static str),
}

impl Expected {
    fn comp(&self, token: &Token) -> bool {
        match self {
            Self::Type(kind) => token.kind == *kind,
            Self::Value(value) => token.value == *value,
        }
    }
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Type(kind) => write!(f, "token of type {}", kind),
            Self::Value(value) => write!(f, "{}", value),
        }
    }
}

fn peek_token_match<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
    expected: Expected,
) -> bool {
    match token_stream.peek() {
        Some(token) => expected.comp(token),
        None => false,
    }
}

fn expect<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
    expected: Expected,
) -> Result<Token, ParseError> {
    let token: Token = match token_stream.next() {
        Some(token) => token,
        None => return Err(ParseError::EOF { expected }),
    };

    if expected.comp(&token) {
        Ok(token)
    } else {
        Err(ParseError::InvalidToken {
            found: token,
            expected: expected,
        })
    }
}

pub enum ParseError {
    EOF { expected: Expected },
    InvalidToken { found: Token, expected: Expected },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EOF { expected } => write!(
                f,
                "Reached end of file without completing valid AST, expected {}",
                expected
            ),
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
            expected: Expected::Value("EOF"),
        }),
    }
}

fn parse_statement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_basestatement(token_stream)?;
    while peek_token_match(token_stream, Expected::Value(";")) {
        tree = AST::new(token_stream.next().unwrap())
            .left(tree)
            .right(parse_basestatement(token_stream)?);
    }
    Ok(tree)
}

fn parse_while_statement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree = AST::new(expect(token_stream, Expected::Value("while"))?)
        .left(parse_expression(token_stream)?);

    expect(token_stream, Expected::Value("do"))?;

    tree = tree.right(parse_statement(token_stream)?);

    expect(token_stream, Expected::Value("endwhile"))?;

    Ok(tree)
}

fn parse_if_statement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree = AST::new(expect(token_stream, Expected::Value("if"))?)
        .left(parse_expression(token_stream)?);

    expect(token_stream, Expected::Value("then"))?;

    tree = tree.middle(parse_statement(token_stream)?);

    expect(token_stream, Expected::Value("else"))?;

    tree = tree.right(parse_statement(token_stream)?);

    expect(token_stream, Expected::Value("endif"))?;

    Ok(tree)
}

fn parse_assignment<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let tree = parse_element(token_stream)?;
    Ok(AST::new(expect(token_stream, Expected::Value(":="))?)
        .left(tree)
        .right(parse_expression(token_stream)?))
}

fn parse_basestatement<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    //basestatement ::= assignment | ifstatement | whilestatement | skip
    if peek_token_match(token_stream, Expected::Value("while")) {
        parse_while_statement(token_stream)
    } else if peek_token_match(token_stream, Expected::Value("if")) {
        parse_if_statement(token_stream)
    } else if peek_token_match(token_stream, Expected::Value("skip")) {
        Ok(AST::new(token_stream.next().unwrap()))
    } else {
        parse_assignment(token_stream)
    }
}

fn parse_expression<I: Iterator<Item = Token>>(
    token_stream: &mut Peekable<I>,
) -> Result<AST, ParseError> {
    let mut tree: AST = parse_term(token_stream)?;

    while peek_token_match(token_stream, Expected::Value("+")) {
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

    while peek_token_match(token_stream, Expected::Value("-")) {
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

    while peek_token_match(token_stream, Expected::Value("/")) {
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

    while peek_token_match(token_stream, Expected::Value("*")) {
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

    if peek_token_match(token_stream, Expected::Value("(")) {
        expect(token_stream, Expected::Value("("))?;
        tree = parse_expression(token_stream)?;
        expect(token_stream, Expected::Value(")"))?;
        Ok(tree)
    } else if peek_token_match(token_stream, Expected::Type(TokenType::Number))
        || peek_token_match(token_stream, Expected::Type(TokenType::Identifier))
    {
        tree = AST::new(token_stream.next().unwrap()); // unwrap valid because peek succeeds
        Ok(tree)
    } else {
        let expected = Expected::Value("( or number or ident");
        Err(ParseError::InvalidToken {
            found: match token_stream.next() {
                Some(token) => token,
                None => return Err(ParseError::EOF { expected }),
            },
            expected,
        })
    }
}
