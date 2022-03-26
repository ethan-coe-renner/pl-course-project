mod parser;
mod scanner;
use crate::scanner::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // read in from input file
    let args: Vec<String> = env::args().collect();

    let inputpath = Path::new(&args[1]);

    let mut inputfile = File::open(&inputpath)?;

    let mut input_text = String::new();
    inputfile.read_to_string(&mut input_text)?;

    let outputpath = Path::new(&args[2]);
    let display = outputpath.display();

    let mut output_file = File::create(&outputpath)?;

    // Scan file to get token stream
    let tokens: Vec<Token> = match scanner::scan_file(input_text) {
        Err(error) => {
            println!("{}", error);
            output_file.write_all(error.to_string().as_bytes())?;
            return Ok(());
        }
        Ok(tokens) => tokens,
    };

    let token_string: String = tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    output_file.write_all(b"Tokens:\n")?;
    output_file.write_all(token_string.as_bytes())?;

    // parse token stream to get AST
    output_file.write_all(b"\n\nAST:\n")?;
    let ast = match parser::parse(&mut tokens.into_iter().peekable()) {
        Ok(ast) => ast,
        Err(error) => {
            println!("Parse Error: {}", error);
            output_file.write_all(error.to_string().as_bytes())?;
            return Ok(());
        }
    };

    let tree_string = ast.to_str(0);

    // Output tokens and AST to output file
    output_file.write_all(tree_string.as_bytes())?;

    println!("Succesfully wrote tokens and tree to {}", display);
    Ok(())
}
