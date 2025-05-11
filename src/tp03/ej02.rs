struct Rectangulo {
    longitud: f64,
    ancho: f64,
}

impl Rectangulo {

    fn new (ancho: f64, longitud: f64) -> Rectangulo {
        Rectangulo{
            longitud,
            ancho,
        }
    }

    fn calcular_area (&self) -> f64 {
        self.longitud * self.ancho
    }

    fn calcular_perimetro (&self) -> f64 {
        2.0 * (self.longitud + self.ancho)
    }

    fn es_cuadrado (&self) -> bool {
        self.longitud == self.ancho
    }
}

//#[should_panic]
#[test]
fn tester() {
    let rectangle = Rectangulo::new(2.0, 2.0);
    assert_eq!(rectangle.calcular_area(), 4.0);
    assert_eq!(rectangle.calcular_perimetro(), 8.0);
    assert_eq!(rectangle.es_cuadrado(), true);
}