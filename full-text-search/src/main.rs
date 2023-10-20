use std::collections::HashMap;

use crate::indexer::calc;

mod indexer;

// todo: stem (NLP)
// todo: multiple word query
// todo: web interface?
// todo: trie for additional search
fn main() {
    let sentences = HashMap::from_iter([
        ("A", "The car is driven on the road"),
        ("B", "The truck is driven on the highway"),
    ]);

    let res = calc(&sentences);
    
    for (w, r) in res.0 {
        println!("{w} -> {r:?}");
    }
}
