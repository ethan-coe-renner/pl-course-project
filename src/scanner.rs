use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum TokenType {
    Identifier,
    Number,
    Symbol,
    Keyword,
}

#[derive(Clone)]
pub struct Token {
    kind: TokenType,
    value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.kind)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type ParseResult<T> = std::result::Result<T, ParseError>;

struct ParseError;

pub fn parse_file(file: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in file.lines() {
        println!("Line: {}", line);
        tokens.append(&mut line_to_tokens(line));
        println!("");
    }
    tokens
}

// converts a line into a vector of tokens
fn line_to_tokens(line: &str) -> Vec<Token> {
    let mut regex_map = HashMap::new(); //map from Token to regex matching that token
    regex_map.insert(
        TokenType::Identifier,
        Regex::new(r"^([[:alpha:]])([[:alpha:]]|[0-9])*$").unwrap(),
    );
    regex_map.insert(TokenType::Number, Regex::new(r"^[0-9]+$").unwrap());
    regex_map.insert(
        TokenType::Symbol,
        Regex::new(r"^(\+|\-|\*|/|\(|\)|;|:=)$").unwrap(),
    );
    regex_map.insert(
        TokenType::Keyword,
        Regex::new(r"^(if|then|else|endif|while|do|endwhile|skip)$").unwrap(),
    );

    let mut tokens: Vec<Token> = Vec::new();

    for word in line.split_whitespace() {
        let mut buffer = String::new(); // buffer of current match
        let mut chars = word.chars().peekable();
        let mut next = chars.next();
        buffer.push(next.unwrap());
        let mut token = match assign_initial_token(&buffer, &regex_map) {
            Ok(t) => t,
            Err(ParseError) => {
                println!("Error reading \"{}\"", buffer);
                return Vec::new();
            }
        };

        next = match chars.peek() {
            None => None,
            Some(&c) => Some(c),
        };

        while next != None {
            buffer.push(next.unwrap());
            if regex_map.get(&token).unwrap().is_match(&buffer) {
                chars.next(); // we can go to next character because this char works
            } else {
                // reached end of token
                buffer.pop();
                match add_token(&buffer, &regex_map, &mut tokens, token) {
                    Err(_) => println!("Error reading \"{}\"", buffer),
                    Ok(kind) => println!("{}: {:?}", buffer, kind),
                }
                buffer = String::new();
                next = chars.next();
                buffer.push(next.unwrap());
                token = match assign_initial_token(&buffer, &regex_map) {
                    Ok(t) => t,
                    Err(ParseError) => {
                        println!("Error reading \"{}\"", buffer);
                        return Vec::new();
                    }
                };
            }
            next = match chars.peek() {
                None => None,
                Some(&c) => Some(c),
            };
        }
        match add_token(&buffer, &regex_map, &mut tokens, token) {
            Err(_) => println!("Error reading \"{}\"", buffer),
            Ok(kind) => println!("{}: {:?}", buffer, kind),
        }
    }
    tokens
}

fn add_token(
    buffer: &str,
    regex_map: &HashMap<TokenType, Regex>,
    tokens: &mut Vec<Token>,
    kind: TokenType,
) -> ParseResult<TokenType> {
    if regex_map.get(&kind).unwrap().is_match(buffer) {
        let newkind = check_if_keyword(buffer, regex_map, kind);
        let newtoken = Token {
            kind: newkind.clone(),
            value: buffer.to_string(),
        };
        tokens.push(newtoken);
        return Ok(newkind);
    } else {
        return Err(ParseError);
    }
}

// checks a complete buffer to see if it is a keyword
fn check_if_keyword(
    buffer: &str,
    regex_map: &HashMap<TokenType, Regex>,
    kind: TokenType,
) -> TokenType {
    if kind == TokenType::Identifier && regex_map.get(&TokenType::Keyword).unwrap().is_match(buffer)
    {
        TokenType::Keyword
    } else {
        kind
    }
}

// takes a char (usually will only have one character) and finds the token which could start with that buffer
// If none match, returns None
fn assign_initial_token(
    buffer: &str,
    regex_map: &HashMap<TokenType, Regex>,
) -> ParseResult<TokenType> {
    if regex_map
        .get(&TokenType::Identifier)
        .unwrap()
        .is_match(buffer)
    {
        Ok(TokenType::Identifier)
    } else if regex_map.get(&TokenType::Number).unwrap().is_match(buffer) {
        Ok(TokenType::Number)
    } else if regex_map.get(&TokenType::Symbol).unwrap().is_match(buffer) {
        Ok(TokenType::Symbol)
    } else if buffer == ":" {
        Ok(TokenType::Symbol)
    } else {
        Err(ParseError)
    }
}
