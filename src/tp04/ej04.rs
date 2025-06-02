#[derive(Clone, Debug)]
struct Producto {
    nombre: String,
    categoria: Categoria,
    precio_base: f64,
}

#[derive(Clone, Debug)]
enum Categoria {
    Hogar,
    Limpieza,
    Comida,
    Tecnologia,
}

#[derive(Clone, Debug)]
struct DatosPersona {
    nombre: String, 
    apellido: String,
    direccion: String,
    dni: u32,
}

#[derive(Clone, Debug)]
struct Vendedor {
    datos: DatosPersona,
    legajo: u32,
    antiguedad: u32,
    salario: f64,
}

#[derive(Clone, Debug)]
struct Cliente {
    datos: DatosPersona,
    suscripcion_newsletter: bool,
    email_suscripcion: Option<String>, 
}

#[derive(Clone, Debug)]
struct VentaProducto {
    producto: Producto,
    cantidad: u32,
}

#[derive(Clone, Debug)]
struct Venta {
    fecha: String,
    cliente: Cliente,
    vendedor: Vendedor,
    productos: Vec<VentaProducto>,
    medio_pago: MedioPago,
}

#[derive(Clone, Debug)]
enum MedioPago {
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
    Efectivo,
}

#[derive(Clone, Debug)]
struct SistemaVentas {
    ventas: Vec<Venta>,
    categorias_descuento: Vec<Categoria>,
}

impl SistemaVentas {
    pub fn new() -> SistemaVentas {
        SistemaVentas {
            ventas: Vec::new(),
            categorias_descuento: vec![Categoria::Hogar, Categoria::Tecnologia],
        }
    }

    pub fn crear_venta(&mut self, fecha: String, cliente: &Cliente, vendedor: &Vendedor, productos: Vec<VentaProducto>, medio_pago: MedioPago) -> Venta {
        let venta = Venta{
            cliente: cliente.clone(),
            fecha: fecha,
            vendedor: vendedor.clone(),
            productos: productos,
            medio_pago: medio_pago,
        };

        self.ventas.push(venta.clone());
        venta
    }



        
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_crear_venta() {
        let persona1 = DatosPersona {
            apellido: "ape".to_string(),
            nombre: "nom".to_string(),
            direccion: "arg".to_string(),
            dni: 123,
         };
         
         let persona2 = DatosPersona {
            apellido: "ape".to_string(),
            nombre: "nom".to_string(),
            direccion: "arg".to_string(),
            dni: 234,
         };

        let cliente = Cliente {datos: persona1, suscripcion_newsletter: true, email_suscripcion: Some(String::from("pepe@email")) };
        let vendedor = Vendedor { datos: persona2, legajo: 0000, antiguedad: 3, salario: 50000.0 };

        let producto1 = Producto {
            nombre: "Agua".to_string(),
            categoria: Categoria::Comida,
            precio_base: 600.0,
        };

        let producto2 = Producto {
            nombre: "Lampara".to_string(),
            categoria: Categoria::Hogar,
            precio_base: 2000.0,
        };

        let prodCant1 = VentaProducto {
            producto: producto1,
            cantidad: 1,
        };

        let prodCant2 = VentaProducto {
            producto: producto2,
            cantidad: 1,
        };

        let productos = vec![prodCant1.clone(), prodCant2.clone()];

        let mut sistema_ventas:SistemaVentas;

        let mut sistema_ventas = SistemaVentas::new();
        assert_eq!(sistema_ventas.ventas.len(), 0); //Ok.

        sistema_ventas.crear_venta("1/1/2025".to_string(), &cliente, &vendedor, productos, MedioPago::TransferenciaBancaria);

        assert_eq!(sistema_ventas.ventas.len(), 1); //Ok.
    }

}