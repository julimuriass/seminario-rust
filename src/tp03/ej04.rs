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

    fn determine_type (&self) -> String {
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

    fn calculate_area (&self) -> f64 {
        let s = (self.side1 + self.side2 + self.side3) / 2.0; // Semi-perimeter
        let area = (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt(); // Heron's formula
        area
    }

    fn calculate_perimeter (&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

//#[should_panic]
#[test]
fn tester() {
    let triangle = Triangle::new(2.0, 2.0, 2.0);
    assert_eq!(triangle.determine_type(), String::from("Equilateral"));
    assert_eq!(triangle.calculate_perimeter(), 6.0);

}