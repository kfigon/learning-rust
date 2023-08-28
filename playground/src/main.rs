use std::{
    io::{prelude::*, BufReader, Error},
    net::{TcpListener, TcpStream}, thread::{self},
};

mod methods;
mod bencode;
mod fan_out;
mod iterators;
mod linked_list;
mod splitter;
mod trie;
mod graph;
mod simple_lexer;
mod tree;
mod smart_pointers;
mod traits;
mod ref_traits;
mod closures;
mod indexer;
mod cli_parse;
mod io;

fn main() {
    println!("hello");

    let listener = TcpListener::bind("0.0.0.0:8080").expect("failed to open tcp connection");
    loop {
        let con = listener.accept();
        thread::spawn(move || {
            match con {
                Err(err) => println!("error reading connection {}", err),
                Ok((stream, _)) => {
                    if let Err(err) = handle(stream) {
                        println!("error sending response {}", err)
                    }
                },
            }
        });
    }
}

fn handle(mut stream: TcpStream) -> Result<(), Error> {
    let mut http_request = Vec::new();
    for line_res in BufReader::new(&stream).lines() {
        let line = line_res?;
        if line.is_empty() {
            break;
        } 
        http_request.push(line);
    }

    println!("{http_request:?}");

    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
    stream.flush()?;

    Ok(())
}