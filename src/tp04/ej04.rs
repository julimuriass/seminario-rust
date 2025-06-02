use std::{ptr::eq, vec};

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

impl PartialEq for Categoria {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Categoria::Comida, Categoria::Comida) => true,
            (Categoria::Hogar, Categoria::Hogar) => true,
            (Categoria::Limpieza, Categoria::Limpieza) => true,
            (Categoria::Tecnologia, Categoria::Tecnologia) => true,
            _ => false,
        }
    }
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
    porcentajes_descuento: Vec<(Categoria, f64)>,
}

impl SistemaVentas {
    pub fn new() -> SistemaVentas {
        SistemaVentas {
            ventas: Vec::new(),
            categorias_descuento: vec![Categoria::Hogar, Categoria::Tecnologia],
            porcentajes_descuento: vec![(Categoria::Hogar, 15.0), (Categoria::Tecnologia, 20.0)],
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

    pub fn obtener_porcentaje_descuento(&self, categoria: &Categoria) -> f64 {
        //Buscar la categoria en el vector de descuentos y retornar lo que esté en la posición 1 de la tupla.
        if let Some(entry) = self.porcentajes_descuento.iter().find(|(desc, _)| *desc == *categoria) {
            return entry.1;
        } else { //Si no está la ctegoría
            return 0.0;
        }
    }

    pub fn obtener_descuento_suscripcion(&self) -> f64 {
        5.0
    }

    pub fn precio_final_venta(&self, venta: &Venta) -> f64 {
        if venta.productos.is_empty() {
            return 0.0;
        }

        let mut precio = 0.0;

        venta.productos.iter()
            .for_each(|p| { //Para cada producto:
                let mut precio_producto = p.producto.precio_base; //Inicializo el precio del producto con su precio base.
                //Tengo que ver si encuentro la categoría del producto en el vector de categorías con descuento del sistema de ventas.
                if self.categorias_descuento.iter().any(|c| *c == p.producto.categoria) {
                    //Si el producto es de una categoría con descuento se lo aplico:
                    precio_producto = (self.obtener_porcentaje_descuento(&p.producto.categoria)/100.0) * p.producto.precio_base;
                }
                precio_producto = precio_producto * p.cantidad as f64; //Multiplico el precio (con el descuento aplicado o no) por la cantidad del producto.
                precio += precio_producto; //Lo sumo en mi acumulador total.
            });
        
        if venta.cliente.suscripcion_newsletter {
            precio -= (self.obtener_descuento_suscripcion()/100.0) * precio;
        }

        precio
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

    #[test]
    fn test_precio_final_venta() {

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
        let mut sistema_ventas = SistemaVentas::new();
        let venta = sistema_ventas.crear_venta("1/1/2025".to_string(), &cliente, &vendedor, productos, MedioPago::TransferenciaBancaria);

        //El precio total tendría que ser de 855.0
        assert_eq!(sistema_ventas.precio_final_venta(&venta), 855.0);

    }

}