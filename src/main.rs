use std::env::{args, Args};

use startup::read_file;

mod lexer;
mod startup;

#[derive(Debug)]
struct ArgsParserError {
    error: String,
}

fn getPath(args: Args) -> Result<String, ArgsParserError> {
    let args: Vec<String> = args.collect();
    if args.len() != 2 {
        return Err(ArgsParserError {
            error: String::from("invalid arguments"),
        });
    }
    Ok(args[1].clone())
}

fn main() {
    let path = getPath(args()).unwrap();
    println!("{}", read_file(path).unwrap());
}
