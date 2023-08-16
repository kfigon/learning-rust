use std::{iter::Iterator, collections::HashMap};

struct LimitedIterator{
    limit: i32,
    i: i32,
}
impl LimitedIterator {
    fn new(i: i32) -> Self {
        Self { limit: i, i: 0 }
    }
}

impl Iterator for LimitedIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.i >= self.limit {
            None
        } else {
            let to_ret = self.i;
            self.i += 1;
            Some(to_ret)
        }
    }
}

struct WrappingIterator {
    limit: i32,
    i: i32,
}

impl WrappingIterator {
    fn new(i: i32) -> Self {
        Self { limit: i, i: 0 }
    }
}

impl Iterator for WrappingIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.i = self.i % self.limit;
        let to_ret = self.i;
        self.i += 1;

        Some(to_ret)
    }
}

#[test]
fn limited_iter() {
    let mut i = LimitedIterator::new(4);
    assert_eq!(Some(0), i.next());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
}

#[test]
fn wrapping_iter() {
    let mut i = WrappingIterator::new(4);
    assert_eq!(Some(0), i.next());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());
    
    assert_eq!(Some(0), i.next());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());

    assert_eq!(Some(0), i.next());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());
}

// will work also with std::env::args()
fn collect_args(v: impl Iterator<Item = String>) -> HashMap<String, String> {
    v.skip(1)
    .collect::<Vec<String>>()
    .chunks(2)
    .filter_map(|pair| match pair {
        [key,val] => Some((key.to_owned(), val.to_owned())),
        _ => None
    })
    .collect::<HashMap<String,String>>()
}

#[test]
fn collect_args_empty() {
    let out = collect_args(Vec::<String>::new().into_iter());

    assert_eq!(out, HashMap::new());
}

#[test]
fn collect_args_pairs() {
    let out = collect_args(vec!["the path", "foo", "bar", "asd", "123"].iter().map(|v|v.to_string()));

    assert_eq!(out, HashMap::from_iter(vec![
        ("foo".to_string(), "bar".to_string()),
        ("asd".to_string(), "123".to_string()),
    ]));
}

#[test]
fn collect_args_pairs_with_redundant_key() {
    let out = collect_args(vec!["the path", "foo", "bar", "asd", "123", "skip meh"].iter().map(|v|v.to_string()));

    assert_eq!(out, HashMap::from_iter(vec![
        ("foo".to_string(), "bar".to_string()),
        ("asd".to_string(), "123".to_string()),
    ]));
}