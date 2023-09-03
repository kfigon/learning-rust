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
mod http_server;
mod threads;
mod async_learn;
mod macros;
mod errors;

use std::thread;
use reqwest;

fn main() {
    println!("hello");

    let threads = vec![
        "http://wp.pl",
        "http://wp.pl",
        "http://google.com",
        "http://google.com",
        "http://google.com",
    ].into_iter()
    .map(|url| thread::spawn(move || {
            match reqwest::blocking::get(url.clone()) {
                Ok(v) => Ok(format!("{url} => {}", v.status())),
                Err(e) => Err(e),
            }
        }))
        .collect::<Vec<_>>();
    
    for t in threads {
        let res = t.join().unwrap();
        println!("{res:?}");
    }

    println!("all done");
}
