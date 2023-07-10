#![allow(dead_code)]

struct Splitter<'a,'b> {
    input: &'a str,
    delimiter: &'b str,
}

impl<'a,'b> Splitter<'a,'b> {
    fn new(input: &'a str, delim: &'b str) -> Self {
        Self { input: input, delimiter: delim }
    }

    fn split(&self) -> Vec<&'a str> {
        self.input.split(self.delimiter).collect()
    }
}

fn do_it<'a>(input: &'a str, delim: &str) -> Vec<&'a str> {
    let s = Splitter::new(input, delim);
    s.split()
}

#[test]
fn split_1() {
    let s = Splitter::new("foo bar baz", " ");
    assert_eq!(s.split(), vec!["foo", "bar", "baz"])
}

#[test]
fn split_2() {
    assert_eq!(do_it("foo bar baz", " "), vec!["foo", "bar", "baz"])
}