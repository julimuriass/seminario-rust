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

    fn calcular_area (&self) -> f64 {
        self.length * self.width
    }

    fn calcular_perimetro (&self) -> f64 {
        2.0 * (self.length + self.width)
    }

    fn es_cuadrado (&self) -> bool {
        self.length == self.width
    }
}

//#[should_panic]
#[test]
fn tester() {
    let rectangle = Rectangle::new(2.0, 2.0);
    assert_eq!(rectangle.calcular_area(), 4.0);
    assert_eq!(rectangle.calcular_perimetro(), 8.0);
    assert_eq!(rectangle.es_cuadrado(), true);
}