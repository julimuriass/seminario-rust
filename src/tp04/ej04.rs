use crate::tp03::ej03::Fecha;

enum Categoria {
    Comida, 
    Limpieza, //Tendrá descuento.
    Hogar,
    Tecnologia, //Tendrá descuento.
}

impl Categoria {
    fn descuento(&self) -> Option<f64> {
        match self {
            Categoria::Limpieza => Some(25.0),
            Categoria::Tecnologia => Some(10.0),
            _ => None
        }
    }
}

struct Producto {
    nombre: String,
    precio_base: f64,
    categoria: Categoria,
}

impl Producto {
    fn new(nombre: String, precio_base: f64, categoria: Categoria) -> Producto {
        Producto {
            nombre,
            precio_base,
            categoria,
        }
    }

    fn precio(&self) -> f64 {
        match self.categoria {
            Categoria::Limpieza => self.precio_base * (self.categoria.descuento().unwrap()/100.0), //I know it's sure to use unwrap 'cause I know that it is a category that has some kind of discount.
            Categoria::Tecnologia => self.precio_base * (self.categoria.descuento().unwrap()/100.0),
            _ => self.precio_base,
        }
    }
}

struct DatosPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
}

struct Vendedor {
    info: DatosPersona,
    legajo: u32,
    antiguedad: u32,
    salario: f64,
}

struct Cliente {
}
    

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_producto() {
        let prod0 = Producto::new("Taza".to_string(), 500.0, Categoria::Hogar);
        let prod1 = Producto::new("Lavandina".to_string(), 450.0, Categoria::Limpieza);

        assert_eq!(prod0.precio(), 500.0); //Does not have to change. Ok.
        assert_eq!(prod1.precio(), 112.5); //Has to change. Ok.

        assert_ne!(prod0.precio(), 100.0);
    }
}