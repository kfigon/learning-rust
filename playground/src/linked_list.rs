
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>
}

struct List {
    head: Option<Box<Node>>
}

impl List {
    fn new() -> Self {
        Self { head: None }
    }

    fn add(&mut self, v: i32) {
        let new_node = Some(Box::new(Node{
            val: v,
            next: None,
        }));

        if self.head.is_none() {
            self.head = new_node;
            return;
        }

        let mut last: &mut Box<Node> = self.head.as_mut().unwrap();
        while last.next.is_some() {
            last = last.next.as_mut().unwrap();
        }
        last.next = new_node;
    }

    fn collect(&self) -> Vec<i32> {
        let mut out = Vec::new();

        let mut ptr: Option<&Box<Node>> = self.head.as_ref();
        while ptr.is_some() {
            out.push(ptr.unwrap().val);
            ptr = ptr.unwrap().next.as_ref();
        }

        out
    }
}

#[test]
fn empty_list() {
    let t = List::new();
    assert_eq!(t.collect(), Vec::<i32>::new());
}


#[test]
fn non_empty_list() {
    let mut t = List::new();
    t.add(4);
    t.add(1);
    t.add(7);
    t.add(3);
    assert_eq!(t.collect(), vec![4,1,7,3]);
}