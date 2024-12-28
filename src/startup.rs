use std::env::args;
use std::fs::File;
use std::io::{Error, Read};

#[derive(Debug)]
struct ArgsParserError {
    error_message: String,
}

pub fn get_code() -> String {
    let args = args().collect();
    let path_result = get_path(args);
    let path = match path_result {
        Ok(str) => str,
        Err(args_error) => {
            panic!("{}", args_error.error_message);
        }
    };
    let file_result = read_file(path);
    match file_result {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    }
}

fn get_path(args: Vec<String>) -> Result<String, ArgsParserError> {
    if args.len() != 2 {
        return Err(ArgsParserError {
            error_message: String::from("invalid arguments"),
        });
    }
    Ok(args.last().unwrap().clone())
}

fn read_file(path: String) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut str = String::new();
    file.read_to_string(&mut str)?;

    Ok(str)
}
