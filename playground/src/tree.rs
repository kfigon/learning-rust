struct Node {
    val: i32,
    left: Link,
    right: Link,
}

type Link = Option<Box<Node>>;

struct Tree {
    root: Link,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn add(&mut self, v: i32) {
        let new_node = Some(Box::new(Node {
            val: v,
            left: None,
            right: None,
        }));

        if self.root.is_none() {
            self.root = new_node;
            return;
        }
        let mut ptr = self.root.as_mut().unwrap();
        loop {
            if v < ptr.val {
                if ptr.left.is_none() {
                    ptr.left = new_node;
                    break;
                }
                ptr = ptr.left.as_mut().unwrap();
            } else {
                if ptr.right.is_none() {
                    ptr.right = new_node;
                    break;
                }
                ptr = ptr.right.as_mut().unwrap();
            }
        }
    }

    fn collect(&self) -> Vec<i32> {
        let mut out = vec![];

        fn dfs(n: &Link, out: &mut Vec<i32>) {
            match n {
                None => return,
                Some(d) => {
                    dfs(&d.left, out);
                    out.push(d.val);
                    dfs(&d.right, out);
                }
            }
        }
        dfs(&self.root, &mut out);
        out
    }
}

#[test]
fn empty_tree() {
    let t = Tree::new();
    assert_eq!(t.collect(), Vec::<i32>::new());
}

#[test]
fn non_empty_tree() {
    let mut t = Tree::new();
    t.add(5);
    t.add(1);
    t.add(3);
    t.add(2);
    t.add(4);
    assert_eq!(t.collect(), vec![1, 2, 3, 4, 5]);
}
