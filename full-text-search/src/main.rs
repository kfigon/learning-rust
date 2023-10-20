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

#[test]
fn smoke_test() {
    let sentences = HashMap::from_iter([
        ("A", "The car is driven on the road"),
        ("B", "The truck is driven on the highway"),
    ]);

    let res = calc(&sentences);
    let empty = HashMap::from_iter([
        ("A", 0.0),
        ("B", 0.0)
    ]);
    assert_eq!(&empty, res.0.get("is").unwrap());
    assert_eq!(&empty, res.0.get("the").unwrap());
    assert_eq!(&empty, res.0.get("driven").unwrap());
    assert_eq!(&empty, res.0.get("on").unwrap());

    assert_ne!(&empty, res.0.get("car").unwrap());
    assert_ne!(&empty, res.0.get("truck").unwrap());
    assert_ne!(&empty, res.0.get("road").unwrap());
    assert_ne!(&empty, res.0.get("highway").unwrap());
}

#[test]
fn word_normalization_test() {
    let sentences = HashMap::from_iter([
        ("A", "foo foo, fo-o .foo FOO"),
    ]);

    let res = calc(&sentences);
    let empty = HashMap::from_iter([
        ("A", 0.0),
    ]);
    assert_eq!(res.0.len(), 1);
    assert_eq!(&empty, res.0.get("foo").unwrap());
}