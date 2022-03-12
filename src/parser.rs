use std::fmt;
use crate::Token;
use crate::TokenType;

pub struct TernaryTree<T> {
    pub value: T,
    pub left: Option<Box<TernaryTree<T>>>,
    pub middle: Option<Box<TernaryTree<T>>>,
    pub right: Option<Box<TernaryTree<T>>>

}

impl<T> TernaryTree<T> {
    pub fn new(value: T) -> Self {
	TernaryTree {
	    value,
	    left: None,
	    middle: None,
	    right: None,
	}
    }

    pub fn left(mut self, node: TernaryTree<T>) -> Self {
	self.left = Some(Box::new(node));
        self
    }

    pub fn middle(mut self, node: TernaryTree<T>) -> Self {
	self.middle = Some(Box::new(node));
        self
    }


    pub fn right(mut self, node: TernaryTree<T>) -> Self {
	self.right = Some(Box::new(node));
        self
    }
}

type AST = TernaryTree<Token>;

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"{}",self.value);
	match self.left {
	    Some(ast) => write!(f,"{}",*ast as AST),
	    None => write!(f,"")
	};
	match self.middle {
	    Some(ast) => write!(f,"{}",*ast as AST),
	    None => write!(f,"")
	};
	match self.right {
	    Some(ast) => write!(f,"{}",*ast as AST),
	    None => write!(f,"")
	}
    }
}


fn parse_element<I>(mut token_stream: &I) -> AST
where I: Iterator<Item = Token>{
    let token: Token;
    match token_stream.next() {
	Some(next) => token = next,
	None => panic!("stream empty")
    };

    let tree: AST;

    if token.value == "(" {
	tree = parse_expression(token_stream);
	return tree;
    }
    else if token.kind == TokenType::Number || token.kind == TokenType::Identifier {
	tree = AST::new(token)
    }
    else {
	panic!("Parse error at {}", token.value);
    }

    tree

}

fn parse_piece<I>(mut token_stream: &I) -> AST
where I: Iterator<Item = Token>{
    let tree: AST = parse_element(token_stream);

    let token: Token;
    match token_stream.next() {
	Some(token) => token = token,
	None => panic!("stream empty")
    };

    while token.value == "*" {
	tree = AST::new(token).left(tree).right(parse_element(token_stream));
	    
	match token_stream.next() {
	    Some(token) => token = token,
	    None => panic!("stream empty")
	};
    }

    tree
}

fn parse_factor<I>(token_stream: &I) -> AST
where I: Iterator<Item = Token>{
    let tree: AST = parse_piece(token_stream);

    let token: Token;
    match token_stream.next() {
	Some(token) => token = token,
	None => panic!("stream empty")
    };

    while token.value == "/" {
	tree = AST::new(token).left(tree).right(parse_piece(token_stream));
	
	match token_stream.next() {
	    Some(token) => token = token,
	    None => panic!("stream empty")
	};
    }

    tree
}

fn parse_term<I>(token_stream: &I) -> AST
where I: Iterator<Item = Token>{
    let tree: AST = parse_factor(token_stream);

    let token: Token;
    match token_stream.next() {
	Some(token) => token = token,
	None => panic!("stream empty")
    };

    while token.value == "-" {
	tree = AST::new(token).left(tree).right(parse_factor(token_stream));
	
	match token_stream.next() {
	    Some(token) => token = token,
	    None => panic!("stream empty")
	};
    }

    tree
}

pub fn parse_expression<I>(token_stream: &I) -> AST
where I: Iterator<Item = Token>{
    let tree: AST = parse_term(token_stream);

    let token: Token;
    match token_stream.next() {
	Some(token) => token = token,
	None => panic!("stream empty")
    };

    while token.value == "+" {
	tree = AST::new(token).left(tree).right(parse_term(token_stream));
	
	match token_stream.next() {
	    Some(token) => token = token,
	    None => panic!("stream empty")
	};
    }

    tree
}

