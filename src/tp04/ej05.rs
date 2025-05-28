use std::collections::HashMap;

use crate::tp03::ej03::Fecha;

struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: u32,
    identidad_validada: bool,
    balance_fiat: f64,
    balance_criptomoneda: HashMap<String, f64>, //Nombre_cripto -> cantidad.
}

struct Blockchain {
    nombre: String,
    prefijo: String,
}

struct Criptomoneda { 
    nombre: String,
    prefijo: String,
    listado_blockchains: Vec<Blockchain>,
}

enum TipoTransaccion {
    IngresoFiat,
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    RetiroFiat,
}

enum Medio {
    MercadoPago,
    TransferenciaBancaria,
}

struct Transaccion {
    fecha: Fecha,
    tipo: TipoTransaccion,
    usuario: Usuario,
    monto_fiat: Option<f64>,
    criptomoneda: Option<Criptomoneda>,
    monto_criptomoneda: Option<f64>,
    cotizacion: Option<f64>,
    blockchain: Option<Blockchain>,
    hash: Option<String>,
    medio: Medio,
}

struct PlataformaXYZ {
    usuarios: HashMap<String, Usuario>, //Email -> usuario.
    criptomonedas: HashMap<String, Criptomoneda>, //Nombre_cripto -> cripto.
    transacciones: Vec<Transaccion>,
}

impl PlataformaXYZ {
    fn new (usuarios: HashMap<String, Usuario>, criptomonedas: HashMap<String, Criptomoneda>, transacciones: Vec<Transaccion>) -> PlataformaXYZ {
        PlataformaXYZ {
            usuarios,
            criptomonedas,
            transacciones,
        }
    }

    
}