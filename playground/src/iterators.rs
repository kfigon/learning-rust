#![allow(dead_code)]

use std::iter::Iterator;

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