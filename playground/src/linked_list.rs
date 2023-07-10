struct Node {
    val: i32,
    next: Option<Box<Node>>
}

struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn add(&mut self, v: i32) {
        let new_node = Some(Box::new(Node {
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
        let mut out: Vec<i32> = Vec::new();
        let mut ptr = &self.head;
        while let Some(v) = ptr {
            out.push(v.val);
            ptr = &v.next;
        }
        out
    }
}

#[test]
fn linked_list1() {
    let mut l = LinkedList::new();
    l.add(1);
    l.add(2);
    l.add(5);

    assert_eq!(l.collect(), vec![1,2,5]);
}