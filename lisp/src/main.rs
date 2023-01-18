use std::io::{self, Write};

use crate::lexer::Token;

mod lexer;

fn main() {
    println!("Welcome to Lisp interpreter");
    println!("Type 'quit' to exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let data = read_line();
        let trimmed = data.trim();
        
        if trimmed == "quit" {
            break;
        } else if trimmed != "" {
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