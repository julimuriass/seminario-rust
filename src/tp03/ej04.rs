struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Triangle {
    fn new (side1: f64, side2: f64, side3: f64) -> Triangle {
        Triangle { 
            side1,
            side2,
            side3,
        }
    }

    fn determinar_tipo (&self) -> String {
        if self.side1 == self.side2 && self.side2 == self.side3 {
            String::from("Equilateral")
        } else {
            if (self.side1 == self.side2 && self.side2 != self.side3) || (self.side1 == self.side3 && self.side3 != self.side2) || (self.side2 == self.side3 && self.side2 != self.side1){
                String::from("Isosceles")
            } else {
                String::from("Scalene")
            }
        }
    }

    fn calcular_area (&self) -> f64 {
        let s = (self.side1 + self.side2 + self.side3) / 2.0; // Semi-perimeter
        let area = (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt(); // Heron's formula
        area
    }

    fn calcular_perimetro (&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

//#[should_panic]
#[test]
fn tester() {
    let triangle = Triangle::new(2.0, 2.0, 2.0);
    assert_eq!(triangle.determinar_tipo(), String::from("Equilateral"));
    assert_eq!(triangle.calcular_perimetro(), 6.0);

}