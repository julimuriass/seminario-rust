struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {

    fn new (width: f64, length: f64) -> Rectangle {
        Rectangle{
            length,
            width,
        }
    }

    fn calculate_area (&self) -> f64 {
        self.length * self.width
    }

    fn calculate_perimeter (&self) -> f64 {
        2.0 * (self.length + self.width)
    }

    fn is_square (&self) -> bool {
        self.length == self.width
    }
}

//#[should_panic]
#[test]
fn tester() {
    let rectangle = Rectangle::new(2.0, 2.0);
    assert_eq!(rectangle.calculate_area(), 4.0);
    assert_eq!(rectangle.calculate_perimeter(), 8.0);
    assert_eq!(rectangle.is_square(), true);
}