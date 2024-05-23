#[derive(Debug)]
struct Tree<T> {
    table: Vec<Option<T>>,
}

impl<T> Tree<T>
where
    T: Ord,
{
    fn new() -> Tree<T> {
        Tree { table: vec![] }
    }

    fn insert(&mut self, v: T) {
        let new_elem = Some(v);
        let mut id = 0;
        loop {
            match self.table.get_mut(id) {
                None => {
                    if id >= self.table.len() {
                        self.table.resize_with(id + 1, Default::default);
                    }
                }
                Some(Some(el)) if new_elem.as_ref().unwrap() > el => id = Tree::<T>::right(id),
                Some(Some(_)) => id = Tree::<T>::left(id),
                Some(el) => {
                    // Some(None)
                    *el = new_elem;
                    break;
                }
            }
        }
    }

    // 0 1 2 3 4 5 6 7
    //            0
    //       1            2
    //    3    4      5       6
    //  7  8  9 10   11 12  13 14

    fn left(id: usize) -> usize {
        id * 2 + 1
    }
    fn right(id: usize) -> usize {
        (id + 1) * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parametric_test {
        ( $( $name:ident -> ($got:expr, $exp:expr)),* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($got, $exp);
                }
            )*
        };
    }

    parametric_test!(
        left0 -> (Tree::<i32>::left(0), 1),
        left1 -> (Tree::<i32>::left(1), 3),
        left2 -> (Tree::<i32>::left(2), 5),
        left3 -> (Tree::<i32>::left(3), 7),
        left4 -> (Tree::<i32>::left(4), 9),
        left5 -> (Tree::<i32>::left(5), 11),
        left6 -> (Tree::<i32>::left(6), 13),
        right0 -> (Tree::<i32>::right(0), 2),
        right1 -> (Tree::<i32>::right(1), 4),
        right2 -> (Tree::<i32>::right(2), 6),
        right3 -> (Tree::<i32>::right(3), 8),
        right4 -> (Tree::<i32>::right(4), 10),
        right5 -> (Tree::<i32>::right(5), 12),
        right6 -> (Tree::<i32>::right(6), 14)
    );

    #[test]
    fn insert_test() {
        let mut t = Tree::<i32>::new();
        for i in [4, 5, 1, 18, 10] {
            t.insert(i);
        }

        assert_eq!(
            t.table,
            vec![
                Some(4),
                Some(1),
                Some(5),
                None,
                None,
                None,
                Some(18),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(10)
            ]
        );
    }
}
