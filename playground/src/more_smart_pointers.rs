#[cfg(test)]
mod cell_tests {
    use std::cell::*;

    // when we want to mutate things through shared (immutable) reference
    // cell is just place in memory

    // used with Rc to store something in multiple places that can mutate it
    #[test]
    fn basic_cell() {
        // this does not compile, cam be only 1 exclusive reference
        // let c = 123;
        // let x = &mut c;
        // let y = &mut c;

        let c = Cell::new(123);
        let mut x = &c;
        let mut y = &c;

        x.set(3);
        y.set(4);

        // both values under the cell changed
        assert_eq!(4, x.get());
        assert_eq!(4, y.get());
        assert_eq!(4, c.get());
    }

    struct Foo {
        v: Cell<i32>,
    }

    impl Foo {
        // immutable ref to self, but can mutate the data!
        fn set(&self, v: i32) {
            self.v.set(v);
        }

        fn get(&self) -> i32 {
            self.v.get()
        }
    }

    #[test]
    fn cell_struct() {
        let f = Foo { v: Cell::new(123) };
        f.set(3);

        assert_eq!(3, f.get())
    }
}

#[cfg(test)]
mod refcell_tests {
    // same as cell, but we can return references to the data, not copy

    use std::cell::RefCell;

    #[test]
    fn basic_refcell() {
        let r = RefCell::new(String::from("foo"));

        // immutable, shared references, but we can mutate underneath data. This is similar how references
        // work in other languages
        let x = &r;
        let y = &r;

        *x.borrow_mut() = String::from("new val");
        *y.borrow_mut() = String::from("even newer val");

        assert_eq!("even newer val", *r.borrow());
        assert_eq!("even newer val", *x.borrow());
        assert_eq!("even newer val", *y.borrow());
    }

    struct Foo<'a> {
        v: RefCell<&'a i32>,
    }

    impl<'a> Foo<'a> {
        // shared ref, immutable, but we can modify it
        fn set(&self, v: &'a i32) {
            *self.v.borrow_mut() = &v;
        }
    }

    #[test]
    fn refcell_struct() {
        let i = 123;

        let f = Foo {
            v: RefCell::new(&i),
        };
        f.set(&3);

        assert_eq!(i, 123);
        assert_eq!(*f.v.borrow(), &3); // same as cell, but for references
    }
}

// COW - copy on write. A type that encodes wheater we have owned or borrowed data. We can accept both then
// AsRef -
