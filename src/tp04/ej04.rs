use crate::tp03::ej03::Fecha;

//Dudas: Podría hacer un trait Producto, y que producto y productoVenta lo implementen? O no es necesario?
//Más dudas a lo largo del ejercicio.

#[derive(Clone)]
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

#[derive(Clone)]
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
            Categoria::Tecnologia => self.precio_base * (self.categoria.descuento().unwrap()/100.0), //Está bien calculado el descuento? jaj
            _ => self.precio_base,
        }
    }
}

#[derive(Clone)]
struct DatosPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
}


impl DatosPersona {
    fn new (nombre: String, apellido: String, direccion: String, dni: u32) -> DatosPersona {
        DatosPersona {
            nombre,
            apellido,
            direccion,
            dni,
        }
    }
}

#[derive(Clone)]
struct Vendedor {
    info: DatosPersona,
    legajo: u32,
    antiguedad: u32,
    salario: f64,
}

impl Vendedor {
    fn new (info: DatosPersona, legajo: u32, antiguedad: u32, salario: f64) -> Vendedor {
        Vendedor {
            info,
            legajo,
            antiguedad,
            salario,
        }
    }
}

#[derive(Clone)]
struct Cliente {
    info: DatosPersona,
    beneficio: bool,
    email_newsletter: Option<String>, //If the client has the benefit then they'll have an email associated with it.
}

impl Cliente {
    fn new(info: DatosPersona, beneficio: bool, email_newsletter: Option<String>) -> Cliente {
        Cliente {
            info,
            beneficio,
            email_newsletter,
        }
    }

    fn descuento_beneficio_newsletter(&self) -> Option<f64> {
        match self.beneficio {
            true => Some(10.0), //If the client has the benefit then this fn will return the discount.
            false => None,
        }
    }
}

#[derive(Clone)]
enum MedioPago {
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
    Efectivo,
}

#[derive(Clone)]
struct ProductoVenta { //Is it okay to create this??
    producto: Producto,
    cantidad_vendida: u32,
}

impl ProductoVenta {
    fn new (producto: Producto, cantidad_vendida: u32) -> ProductoVenta {
        ProductoVenta {
            producto,
            cantidad_vendida,
        }
    }

    fn precio_total (&self) -> f64 {
        self.producto.precio() * self.cantidad_vendida as f64 
    }
}

#[derive(Clone)]  
struct Venta {
    listado_productos: Vec<ProductoVenta>,
    vendedor: Vendedor,
    cliente: Cliente,
    fecha: Fecha,
    medio_pago: MedioPago,
}

impl Venta {
    fn crear_venta(listado_productos: Vec<ProductoVenta>, vendedor: Vendedor, cliente: Cliente, fecha: Fecha, medio_pago: MedioPago) -> Venta {
        Venta {
            listado_productos,
            vendedor,
            cliente,
            fecha,
            medio_pago,
        }
    }

    fn calcular_precio_final (&self) -> f64 { //Ask if this is okay. No sé si lo estoy haciendo como quieren que lo hagamos en la consigna :/
        let precios_productos:Vec<f64>= self.listado_productos.iter().map(|pv| pv.precio_total()).collect();
        
        let mut total = precios_productos.iter().sum();

        match self.cliente.beneficio {
            true => total = total * (self.cliente.descuento_beneficio_newsletter().unwrap() / 100.0), //Apply the discount.
            false => (),            
        }

        total
    }
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

    #[test]
    fn test_venta_precio_final() {
        let datos_cliente = DatosPersona::new(
            "juli".to_string(),
            "murias".to_string(),
            "arg".to_string(),
            12345678,
        );
        let cliente_con_newsletter = Cliente::new(datos_cliente.clone(), true, Some("jm@email.com".to_string()));
        let cliente_sin_newsletter = Cliente::new(datos_cliente, false, None);

        let datos_vendedor = DatosPersona::new(
            "pepe".to_string(),
            "González".to_string(),
            "arg".to_string(),
            87654321,
        );
        let vendedor = Vendedor::new(datos_vendedor, 1001, 5, 50000.0);

        let producto1 = Producto::new("Detergente".to_string(), 200.0, Categoria::Limpieza);
        let producto2 = Producto::new("Silla".to_string(), 1000.0, Categoria::Hogar);

        //assert_eq!((producto1.clone().precio()*2 as f64 + producto2.clone().precio()), 1100.0); //Ok.


        let productos = vec![
            ProductoVenta::new(producto1, 2), 
            ProductoVenta::new(producto2, 1), 
        ];


        let fecha = Fecha { dia: 27, mes: 5, año: 2025 };

        // Test with newsletter discount
        let venta_con_newsletter = Venta::crear_venta(
            productos.clone(),
            vendedor.clone(),
            cliente_con_newsletter.clone(),
            fecha.clone(),
            MedioPago::Efectivo,
        );


        //assert_eq!(venta_con_newsletter.calcular_precio_final(), 110.0); //Ok.

        // Test without newsletter discount
        let venta_sin_newsletter = Venta::crear_venta(
            productos.clone(),
            vendedor.clone(),
            cliente_sin_newsletter.clone(),
            fecha,
            MedioPago::Efectivo,
        );

        
        assert_eq!(venta_sin_newsletter.calcular_precio_final(), 1100.0);
    }
}