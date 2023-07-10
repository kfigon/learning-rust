use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
}

#[derive(Debug)]
struct Trie {
    root: Node,
}

impl Node {
    fn new() -> Self {
        Self { children: HashMap::new(), is_end: false }
    }
}

impl Trie {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn find(&self, s: &str) -> Vec<String> {
        let mut out = vec![];

        fn traverse(n: &Node, current: &str, out: &mut Vec<String>) {
            if n.is_end {
                out.push(String::from(current));
            }

            for c in &n.children {
                traverse(c.1, &format!("{}{}", current, c.0), out);
            }
        }

        let mut n = &self.root;
        for c in s.chars() {
            n = match n.children.get(&c) {
                None => return out,
                Some(v) => v,
            };
        }

        traverse(&n, s, &mut out);
        out
    }

    fn add(&mut self, s: &str) {
        let mut n = &mut self.root;

        for (i, c) in  s.char_indices() {
            let new_node = n.children.entry(c).or_insert(Node::new());
            if i == s.len() -1 {
                new_node.is_end = true;
            }
            n = new_node;
        }
    }
}

#[test]
fn empty_trie() {
    let t = Trie::new();
    assert_eq!(t.find("foo"), Vec::<String>::new())
}

#[test]
fn single_trie() {
    let mut t = Trie::new();
    t.add("foobar");

    assert_eq!(t.find("foo"), vec!["foobar"]);
    assert_eq!(t.find("f"), vec!["foobar"]);
    assert_eq!(t.find("as"), Vec::<String>::new());
}

#[test]
fn multiple_trie() {
    let mut t = Trie::new();
    t.add("apple");
    t.add("ape");
    t.add("hi");
    t.add("hello");
    t.add("hell");
    
    assertEq(t.find("a"), vec!["apple", "ape"]);
    assertEq(t.find("ap"), vec!["apple", "ape"]);
    assertEq(t.find("app"), vec!["apple"]);
    assertEq(t.find("h"), vec!["hi", "hell", "hello"]);
    assertEq(t.find("hell"), vec!["hell", "hello"]);
    assertEq(t.find("hello"), vec!["hello"]);
    assertEq(t.find(""), vec!["apple", "ape", "hi", "hello", "hell"]);
}

fn assertEq(first: Vec<String>, second: Vec<&str>) {
    assert_eq!(first.len(), second.len());
    first.iter().for_each(|v| assert!(second.contains(&v.as_str())));
    second.iter().for_each(|&v| assert!(first.contains(&String::from(v))));
}