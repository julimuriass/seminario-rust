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
        2 * (self.length + self.width)
    }

    fn is_square (&self) -> f64 {
        self.length == self.width
    }
}