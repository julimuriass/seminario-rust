use chrono::{DateTime, Utc};

struct Producto {
    nombre: String,
    categoria: Categoria,
    precio_base: f64,
}

enum Categoria {
    Hogar,
    Limpieza,
    Comida,
    Tecnologia,
}

struct DatosPersona {
    nombre: String, 
    apellido: String,
    direccion: String,
    dni: u32,
}

struct Vendedor {
    datos: DatosPersona,
    legajo: u32,
    antiguedad: u32,
    salario: f64,
}

struct Cliente {
    suscripcion_newsletter: bool,
    email_suscripcion: Option<String>, 
}

struct VentaProducto {
    producto: Producto,
    cantidad: u32,
}

struct Venta {
    fecha: DateTime<Utc>,
    cliente: Cliente,
    vendedor: Vendedor,
    productos: Vec<VentaProducto>,
    medio_pago: MedioPago,
}

enum MedioPago {
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
    Efectivo,
}

struct SistemaVentas {
    ventas: Vec<Venta>,
    categorias_descuento: Vec<Categoria>,
}