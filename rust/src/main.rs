use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, process};

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use config::Config;
use lexer::Lexer;

mod config;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Cannot parse arguments: {}", err);
        process::exit(1);
    });
    let mut file = File::open(Path::new(config.path)).unwrap_or_else(|err| {
        eprintln!("Cannot open file: {}", err);
        process::exit(2);
    });

    let mut string = String::new();
    file.read_to_string(&mut string).unwrap_or_else(|err| {
        eprintln!("Cannot read file: {}", err);
        process::exit(3);
    });

    let mut lexer = Lexer::new(string.as_str());
    lexer.lex().unwrap_or_else(|err| {
        eprintln!("Cannot lex file: {}", err);
        process::exit(4);
    });

    let mut parser = Parser::new(lexer.tokens);
    let ast = parser.parse().unwrap_or_else(|err| {
        eprintln!("Cannot parse AST: {}", err);
        process::exit(5);
    });
    Interpreter::new().run(&ast);
}
