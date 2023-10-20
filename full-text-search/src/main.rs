use std::collections::HashMap;

use crate::indexer::calc;

mod indexer;

// todo: stem (NLP)
// todo: multiple word query
// todo: web interface?
// todo: trie for additional search
fn main() {
    let sentences = HashMap::from_iter([
        ("fileA", "The car is driven on the road"),
        ("fileB", "The truck is driven on the highway"),
    ]);

    let res = calc(&sentences);
    
    for (w, r) in &res.0 {
        println!("{w} -> {r:?}");
    }

    println!();
    let get = |word: &str| res.0.get(word).filter(|v| v.iter().any(|(_, ranking)| *ranking > 0.0));
    println!("car -> {:?}", get("car"));
    println!("truck -> {:?}", get("truck"));
    println!("the -> {:?}", get("the"));
}
