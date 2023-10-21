use std::env;
use std::fs;
use std::io::{self, Write};

use crate::lexer::lex;
use crate::parser::parse;

mod lexer;
mod parser;
mod evaluator;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    match args.len() {
        0 => interpreter_mode(),
        1 => file_mode(args.get(0).unwrap()),
        v => println!("Invalid numer of arguments {}, exiting", v),
    }
}

fn file_mode(file_name: &str) {
    match fs::read_to_string(file_name) {
        Ok(file_content) => println!("{:?}", parse(lex(&file_content))),
        Err(error) => println!("error opening file {file_name}: {error}"),
    }
}

fn interpreter_mode() {
    println!("Welcome to Lisp interpreter");
    println!("Type 'quit' to exit");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let data = read_line();
        let trimmed = data.trim();

        if trimmed == "quit" {
            break;
        } else if !trimmed.is_empty() {
            print!("{}", data);
        }
    }
    println!("Goodbye")
}

fn read_line() -> String {
    let mut out = String::new();
    io::stdin()
        .read_line(&mut out)
        .expect("Failed to read line");

    out
}
