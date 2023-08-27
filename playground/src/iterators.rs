use std::iter::Iterator;

// IntoIterator trait - used by for loops to generate iterator
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

#[test]
fn owned_into_iter_test() {
    let v = vec![2,3,4,5];
    assert_eq!(v.into_iter().collect::<Vec<_>>(), vec![2,3,4,5]);
    // assert_eq!(v.len(), 4); compile error - borrow of moved value
}

#[test]
fn borrowed_into_iter_test() {
    let v = vec![2,3,4,5];
    // into iter on borrowed produces borrowed items
    assert_eq!((&v).into_iter().collect::<Vec<_>>(), vec![&2,&3,&4,&5]);
    assert_eq!(v.len(), 4); // all ok, does not value

    // (&mut v).iter_mut() // produces mutable borrowed values and 

    // (&v).into_iter() is clunky, so it's easier to use iter() for borrowing
    // v.iter()
}

#[test]
fn flatten_test() {
    let v = vec![
        vec![2,3,4],
        vec![5,6,7],
    ];

    // flats nested iters

    // into_iter because I dont want to deal with references
    assert_eq!(v.into_iter().flatten().collect::<Vec<_>>(), vec![2,3,4,5,6,7]);
}

#[test]
fn flatmap_test() {
    let v = vec![
        ("foo", vec![2,3,4]),
        ("bar", vec![5,6,7]),
    ];

    // flat with some additional mapping for nested iters
    assert_eq!(v.into_iter().flat_map(|el| el.1).collect::<Vec<_>>(), vec![2,3,4,5,6,7]);
}

#[test]
fn flatmap_options_test() {
    let v = vec![
        Some("foo"), None, None, Some("bar"), None, Some("asd")
    ];

    // flatten on Option skips nons. Same for Results
    assert_eq!(v.into_iter().flatten().collect::<Vec<_>>(), vec!["foo", "bar", "asd"]);
}

#[test]
fn flatmap_characters_test() {
    assert_eq!(
        "this is a string"
            .chars()
            .flat_map(char::to_uppercase) // to_uppercase returns an iterator of multiple chars. Flat them
            .collect::<String>(),
        "THIS IS A STRING"
    );
}

#[test]
fn chain_test() {
    // append iterators
    let a = vec![1,2,3].into_iter();
    let b = vec![2,3,4].into_iter();

    assert_eq!(a.chain(b).collect::<Vec<_>>(), vec![1,2,3,2,3,4]);
}

#[test]
fn cloned_test() {
    let a = vec![2,3,4,5];

    // borrowed
    assert_eq!(a.iter().collect::<Vec<_>>(), vec![&2,&3,&4,&5]);
    
    // get cloned values, so we dont need to deal with references
    assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![2,3,4,5]);
}

#[test]
fn zip_test() {
    let even = vec![2,4,6,8].into_iter();
    let odd = vec![1,3,5].into_iter();

    // produce iterator with both combined, one after another
    // ends when any sub iter ends

    assert_eq!(odd.zip(even).flat_map(|v| [v.0, v.1]).collect::<Vec<_>>(), vec![1,2,3,4,5,6]);
}

// some iterator adaptors (map, take_while etc) takes ownership of iterator
// we can pass a reference with .by_ref() to reuse the same iterator in many places