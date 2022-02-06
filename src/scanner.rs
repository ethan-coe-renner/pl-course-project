use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug)]
enum TokenType {
    Identifier,
    Number,
    Symbol,
}

pub struct Token {
    kind: TokenType,
    value: String
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
    regex_map.insert(TokenType::Symbol, Regex::new(r"^(\+|\-|\*|/|\(|\))$").unwrap());

    let mut tokens: Vec<Token> = Vec::new();

    for word in line.split_whitespace() {
        let mut buffer = String::new(); // buffer of current match
        let mut chars = word.chars().peekable();
        let mut next = chars.next();
        buffer.push(next.unwrap());
        let mut token = match assign_initial_token(&buffer, &regex_map) {
            None => {
                println!("Error reading \"{}\"", buffer);
                return Vec::new();
            }
            Some(t) => t,
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
                println!("{}: {:?}", buffer, &token);
                tokens.push(Token {kind: token, value: buffer});
                buffer = String::new();
                next = chars.next();
                buffer.push(next.unwrap());
                token = match assign_initial_token(&buffer, &regex_map) {
                    None => {
                        println!("Error reading \"{}\"", buffer);
                        return Vec::new();
                    }
                    Some(t) => t,
                };
            }
            next = match chars.peek() {
                None => None,
                Some(&c) => Some(c),
            };
        }
        println!("{}: {:?}", buffer, &token);
        tokens.push(Token {kind: token, value: buffer});
    }
    tokens
}

// takes a char (ussually will only have one character) and finds the token which chould start with that buffer
// If none match, returns None
fn assign_initial_token(buffer: &str, regex_map: &HashMap<TokenType, Regex>) -> Option<TokenType> {
    if regex_map.get(&TokenType::Identifier).unwrap().is_match(buffer) {
        Some(TokenType::Identifier)
    } else if regex_map.get(&TokenType::Number).unwrap().is_match(buffer) {
        Some(TokenType::Number)
    } else if regex_map.get(&TokenType::Symbol).unwrap().is_match(buffer) {
        Some(TokenType::Symbol)
    } else {
        None
    }
}
