#[cfg(test)]
mod test {
    use std::{cell::Cell, rc::Rc};

    struct Data {
        a: String,
        b: Vec<Rc<Data>>,
    }

    impl Data {
        fn new(v: &str) -> Self {
            Self {
                a: String::from(v),
                b: vec![],
            }
        }
    }

    #[test]
    fn box_test() {
        // basic heap allocation, pointer indirection (known size)
        let b = Box::new(Data::new("data"));
        assert_eq!(b.a, "data".to_string());
    }

    #[test]
    fn rc_test() {
        // ARC - same, but atomic rc - thread safe

        // many owners for the same data
        // no need for lifetimes. Also sometime lifetimes are hard to figure, this is much easier
        let base = Rc::new(Data::new("data"));
        let mut owner1 = Data::new("1");
        let mut owner2 = Data::new("2");

        // clone does not copy the data, just bumps the counter
        owner1.b.push(base.clone());
        owner2.b.push(base.clone());
        assert_eq!(
            owner1.b.iter().map(|v| v.a.as_str()).collect::<Vec<&str>>(),
            vec!["data"]
        );
        assert_eq!(
            owner2.b.iter().map(|v| v.a.as_str()).collect::<Vec<&str>>(),
            vec!["data"]
        );
        assert_eq!(Rc::strong_count(&base), 3);
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct V(i32, i32, i32);

    struct Owner {
        v: Cell<V>,
    }

    #[test]
    fn cell_test() {
        // for interior mutability - if we have a readonly data, bt ut we still want to mutate it. It's moving borrow checker to the runtime
        // RefCell - for references
        // Cell - for copyable types
        let v = Cell::new(V(1, 2, 3));
        let o = Owner { v }; // v is immutable, but I can still change it

        {
            let mut data = o.v.get();
            data.0 = 5;
            data.1 = 6;
            data.2 = 7;

            o.v.set(data);
        }
        assert_eq!(o.v.get(), V(5, 6, 7))
    }
}
