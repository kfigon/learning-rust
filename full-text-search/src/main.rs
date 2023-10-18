use std::collections::HashMap;

use crate::indexer::calc;

mod indexer;

// todo: stem (NLP), lowercase and remove punctuation
// todo: trie for additional search
// todo: web interface?
fn main() {
    let sentences = vec![
        "The car is driven on the road",
        "The truck is driven on the highway"
    ];

    let res = calc(&sentences);
    
    for (w, r) in res.0 {
        println!("{w} -> {r:?}");
    }
}

#[test]
fn foo() {
    let sentences = vec![
        "The car is driven on the road",
        "The truck is driven on the highway"
    ];

    let res = calc(&sentences);
    let empty: HashMap<usize, f64> = HashMap::from_iter([
        (0, 0.0),
        (1, 0.0)
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