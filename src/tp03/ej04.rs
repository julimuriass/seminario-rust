struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
}

impl Triangulo {
    fn new (lado1: f64, lado2: f64, lado3: f64) -> Triangulo {
        Triangulo { 
            lado1,
            lado2,
            lado3,
        }
    }

    fn determinar_tipo (&self) -> String {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            String::from("Equilateral")
        } else {
            if (self.lado1 == self.lado2 && self.lado2 != self.lado3) || (self.lado1 == self.lado3 && self.lado3 != self.lado2) || (self.lado2 == self.lado3 && self.lado2 != self.lado1){
                String::from("Isosceles")
            } else {
                String::from("Scalene")
            }
        }
    }

    fn calcular_area (&self) -> f64 {
        let s = (self.lado1 + self.lado2 + self.lado3) / 2.0; // Semi-perimeter
        let area = (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt(); // Heron's formula
        area
    }

    fn calcular_perimetro (&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }
}

//#[should_panic]
#[test]
fn tester() {
    let triangle = Triangulo::new(2.0, 2.0, 2.0);
    assert_eq!(triangle.determinar_tipo(), String::from("Equilateral"));
    assert_eq!(triangle.calcular_perimetro(), 6.0);

}