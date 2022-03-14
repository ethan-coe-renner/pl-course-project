mod parser;
mod scanner;
use crate::scanner::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // read in from input file
    let args: Vec<String> = env::args().collect();

    let inputpath = Path::new(&args[1]);
    let display = inputpath.display();

    let mut inputfile = match File::open(&inputpath) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut input_text = String::new();
    match inputfile.read_to_string(&mut input_text) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Successfully read {}\n", display),
    }

    // Scan file to get token stream
    let tokens: Vec<Token> = match scanner::parse_file(input_text) {
        Err(error) => {
            println!("{}", error);
            return;
        }
        Ok(tokens) => tokens,
    };

    let mut token_string: String = tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    token_string.push('\n');
    token_string.push('\n');

    // parse token stream to get AST
    let ast = match parser::parse(&mut tokens.into_iter().peekable()) {
        Ok(ast) => ast,
        Err(error) => {
            println!("Parse Error: {}", error);
            return;
        }
    };

    println!("AST:");
    let tree_string = ast.to_str(0);
    println!("{}", tree_string);

    // Output tokens and AST to output file
    let outputpath = Path::new(&args[2]);
    let display = outputpath.display();

    let mut output_file = match File::create(&outputpath) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match output_file.write_all(token_string.as_bytes()) {
        Err(why) => panic!("couldn't write tokens to {}: {}", display, why),
        Ok(_) => {}
    }

    match output_file.write_all(tree_string.as_bytes()) {
        Err(why) => panic!("couldn't write tree to {}: {}", display, why),
        Ok(_) => {}
    }

    println!("Succesfully wrote tokens and tree to {}", display);
}
