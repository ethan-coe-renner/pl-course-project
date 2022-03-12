mod scanner;
mod parser;
use crate::scanner::*;
use crate::parser::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;


fn main() {
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
        Ok(_) => println!("Successfully read file\n"),
    }

    let tokens = scanner::parse_file(input_text);
    let ast = parser::parse_expression(&mut tokens.into_iter());

    println!("{}",ast);


    // let token_string: String = tokens.iter().map(|token| token.to_string()).collect::<Vec<String>>().join("\n");

    // let outputpath = Path::new(&args[2]);
    // let display = outputpath.display();

    // let mut output_file = match File::create(&outputpath) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // match output_file.write_all(token_string.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
}
