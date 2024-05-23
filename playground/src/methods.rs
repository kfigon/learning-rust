#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn new(w: usize, h: usize) -> Self {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

impl ToString for Rectangle {
    fn to_string(&self) -> String {
        format!("{},{}", self.width, self.height)
    }
}

#[test]
fn rectangle_area() {
    let rec = Rectangle::new(2, 3);
    assert_eq!(6, rec.area());
    assert_eq!(6, rec.area());
}

#[test]
fn rectangle_trait() {
    let rec = Rectangle::new(2, 3);
    assert_eq!("2,3", rec.to_string());
}
