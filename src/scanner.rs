use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Identifier,
    Number,
    Symbol,
}

impl fmt::Display for Token {
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
        Token::Identifier,
        Regex::new(r"^([[:alpha:]])([[:alpha:]]|[0-9])*$").unwrap(),
    );
    regex_map.insert(Token::Number, Regex::new(r"^[0-9]+$").unwrap());
    regex_map.insert(Token::Symbol, Regex::new(r"^(\+|\-|\*|/|\(|\))$").unwrap());

    let mut tokens = Vec::new();

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
                tokens.push(token);
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
        tokens.push(token);
    }
    tokens
}

// takes a char (ussually will only have one character) and finds the token which chould start with that buffer
// If none match, returns None
fn assign_initial_token(buffer: &str, regex_map: &HashMap<Token, Regex>) -> Option<Token> {
    if regex_map.get(&Token::Identifier).unwrap().is_match(buffer) {
        Some(Token::Identifier)
    } else if regex_map.get(&Token::Number).unwrap().is_match(buffer) {
        Some(Token::Number)
    } else if regex_map.get(&Token::Symbol).unwrap().is_match(buffer) {
        Some(Token::Symbol)
    } else {
        None
    }
}
