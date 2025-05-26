use crate::tp03::ej03::Fecha;

struct StreamingRust {
    usuarios: Vec<Usuario>,
}

struct Suscripcion {
    tipo: TipoSuscripcion,
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: Fecha,
    activa: bool,
}

struct Usuario {
    nombre: String,
    email: String,
    username: String,
    suscripciones: Vec<Suscripcion>,
    medio_pago: MedioPago,
}

enum TipoSuscripcion {
    Basic,
    Clasic, 
    Super,
}

enum MedioPago {
    Efectivo,
    MercadoPago,
    TransferenciaBancaria,
    Cripto,
    TarjetaCredito,
}

impl Usuario {
    fn crear_usuario (nombre: String, email: String, username: String, medio_pago: MedioPago, suscripcion: Suscripcion) -> Usuario {
        Usuario {
            nombre,
            email,
            username,
            suscripciones : vec![suscripcion],
            medio_pago,
        }
    } 

    fn suscripcion_activa (&self) -> Option<&Suscripcion> {
        self.suscripciones.iter().find(|s| s.activa == true)
        
    }
}

impl StreamingRust {
    fn upgrade_suscripcion (&mut self, usuario: &Usuario) {
        //Find the user.
        if let Some(user) = self.usuarios.iter_mut().find(|u| compare_users(&u, &usuario)) {
            //Find the subscription I want to upgrade.
            if let Some(){
                match subscription.tipo {
                    TipoSuscripcion::Basic => {subscription.tipo = TipoSuscripcion::Clasic;}
                    TipoSuscripcion::Clasic => {subscription.tipo = TipoSuscripcion::Super;}
                    _ => {}
                }
            } 
        }
    }

    fn downgrade_suscripcion (&mut self, usuario: &Usuario) {
        //Find the user.
        if let Some(user) = self.usuarios.iter_mut().find(|u| compare_users(&u, &usuario)) {
            //Find the subscription I want to upgrade.
            if let Some(subscription) = user.suscripciones.iter_mut().find(|s| s.activa == true) {
                match subscription.tipo {
                    TipoSuscripcion::Basic => {subscription.tipo = TipoSuscripcion::Clasic;}
                    TipoSuscripcion::Clasic => {subscription.tipo = TipoSuscripcion::Super;}
                    _ => {}
                }
            } 
        }
    }
}

pub fn compare_users (user1: &Usuario, user2: &Usuario) -> bool {
    user1.email.eq_ignore_ascii_case(&user2.email) &&
    user1.nombre.eq_ignore_ascii_case(&user2.nombre) &&
    comparar_medio_pagos (&user1.medio_pago, &user2.medio_pago)
}

pub fn comparar_medio_pagos (medio1: &MedioPago, medio2: &MedioPago) -> bool {
    match (medio1, medio2) {
        (MedioPago::Cripto, MedioPago::Cripto) => true,
        (MedioPago::Efectivo, MedioPago::Efectivo) => true,
        (MedioPago::MercadoPago, MedioPago::MercadoPago) => true,
        (MedioPago::TarjetaCredito, MedioPago::TarjetaCredito) => true,
        (MedioPago::TransferenciaBancaria, MedioPago::TransferenciaBancaria) => true,
        (_) => false,
    }
}

