use std::collections::HashMap;
use crate::tp03::ej03::Fecha;

enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

impl TipoSuscripcion {
    fn costo_mensual(&self) -> f64 {
        match self {
            TipoSuscripcion::Basic => 5.0,
            TipoSuscripcion::Clasic => 9.5,
            TipoSuscripcion::Super => 15.0,
        }
    }

    fn upgrade(&self) -> Option<TipoSuscripcion>{
        match self {
            TipoSuscripcion::Basic => Some(TipoSuscripcion::Clasic),
            TipoSuscripcion::Clasic => Some(TipoSuscripcion::Super),
            TipoSuscripcion::Super => None,
        }
    }

    fn downgrade(&self) -> Option<TipoSuscripcion> {
        match  self {
            TipoSuscripcion::Basic => None,
            TipoSuscripcion::Clasic => Some(TipoSuscripcion::Basic),
            TipoSuscripcion::Super => Some(TipoSuscripcion::Clasic),
            
        }
    }

    //Tendría que implementar un trait para los soy_...() ???!!!
    fn soy_basic(&self) -> bool {
        match self {
            TipoSuscripcion::Basic => true,
            _ => false, 
        }
    }

    fn soy_clasic(&self) -> bool {
        match self {
            TipoSuscripcion::Clasic => true,
            _ => false, 
        }
    }

    fn soy_super(&self) -> bool {
        match self {
            TipoSuscripcion::Super => true,
            _ => false, 
        }
    }
}

struct Suscripcion {
    tipo: TipoSuscripcion,
    duracion_meses: u8,
    fecha_inicio: Fecha,
    activa: bool,
}

impl Suscripcion {
    fn new(tipo: TipoSuscripcion, duracion_meses: u8, fecha_inicio: Fecha) -> Suscripcion {
        Suscripcion {
            tipo,
            duracion_meses,
            fecha_inicio,
            activa: true,
        }
    }

    fn activar_suscripcion(&mut self) {
        self.activa = true
    }

    fn desactivar_suscripcion(&mut self) {
        self.activa = false
    }

    fn upgrade(&mut self) -> Result<(), String> {
        match self.tipo.upgrade() {
            Some(tipo_nuevo) => {
                self.tipo = tipo_nuevo;
                Ok(())
            }
            None => Err("No se puede ascender de Super.".to_string()) //Is it okay to make it an err? 
        }
    }

    fn downgrade(&mut self) -> Result<(), String> {
        match self.tipo.downgrade() {
            Some(tipo_nuevo) => {
                self.tipo = tipo_nuevo;
                Ok(())
            }
            None => {
                self.desactivar_suscripcion();
                Err("Se ha canclado su suscripción.".to_string())
            }
        }
    }
}

enum MedioPago {
    Efectivo,
    MercadoPago {
        cbu: u32,
    },
    TransferenciaBancaria {
        cuenta_destino: String,
        cuenta_origen: String,    },
    TarjetaCredito {
        numero_tarjeta: u32,
    },
    Cripto {
        tipo_cripto: String,
    },
}

struct Usuario {
    suscripciones: Vec<Suscripcion>,
    medio_pago: MedioPago,
    id: u32,
    username: String,
    nombre: String,
    apellido: String,
    email: String,
}

impl Usuario {
    fn new(medio_pago: MedioPago, id: u32, username: String, nombre: String, apellido: String, email: String) -> Usuario {
        Usuario {
            medio_pago,
            suscripciones: Vec::new(),
            id,
            username,
            nombre,
            apellido,
            email,
        }
    }

    fn agregar_suscripcion(&mut self, suscripcion: Suscripcion) {
        self.suscripciones.iter_mut().for_each(|s| s.desactivar_suscripcion()); //Deactivate all previous subscriptons.
        self.suscripciones.push(suscripcion); //Add the new subscription.
    }

    fn obtener_suscripcion_activa(&self) -> Option<&Suscripcion> {
        self.suscripciones.iter().find(|s| s.activa)
    }

    fn obtener_suscripcion_activa_mutable(&mut self) -> Option<&mut Suscripcion> {
        self.suscripciones.iter_mut().find(|s| s.activa)
    }

    fn cancelar_suscripcion(&mut self) -> Result<(), String> {
        match self.obtener_suscripcion_activa_mutable() {
            Some(suscripcion_a_cancelar) => {
                suscripcion_a_cancelar.desactivar_suscripcion();
                Ok(())
            }
            None => Err("No se puede cancelar esa suscripción. Ya sea porque no existe o porque ya está desactivada.".to_string())
        }
    }

    fn upgrade_suscripcion(&mut self) -> Result<(), String> {
        match self.obtener_suscripcion_activa_mutable() {
            Some(suscripcion) => {
                suscripcion.upgrade();
                Ok(())
            }
            None => Err("No hay una suscripción activa para mejorar.".to_string())
        }
    }

    fn downgrade_suscripcion(&mut self) -> Result<(), String> {
        match self.obtener_suscripcion_activa_mutable() {
            Some(suscripcion) => {
                suscripcion.downgrade();
                Ok(())
            }
            None => Err("No hay una suscripción activa para degradar.".to_string())
        }
    }
}

struct StreamingRust {
    usuarios: Vec<Usuario>, //Key is the user's id.
}

impl StreamingRust {
    /*fn medio_pago_mas_usado_suscripciones_activas(&self) -> Option<TipoSuscripcion> {
        //Suscripciones_activas is a vec containing references to the TipoSuscripcion.
        //...= values goes through each element of the hashmap. Filter_map is going to filter and transform the following:  Map is going to transform all the elements into its types (only the elements that have an active subscription). Collect is going to 'collect' and 'put' that all into the Vec.
        let suscripciones_activas: Vec<&TipoSuscripcion> = self.usuarios.values().filter_map(|u| u.obtener_suscripcion_activa().map(|s|&s.tipo)).collect();

        if suscripciones_activas.is_empty() {
            return None;
        } else {
            //Ask for help!!!!
            //Es mucho lío recorrer suscripciones_activas y tener un arreglo auxiliar para acumular la cantidad 
        }
    }*/

    fn medio_pago_mas_usado_suscripciones_activas(&self) -> Option<TipoSuscripcion> {
        if self.usuarios.is_empty() {
            return None;
        } else {
            let 
        }

    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_upgrade_downgrade() {
        let mut suscripcion0 = Suscripcion::new(TipoSuscripcion::Basic, 3, Fecha { dia: (12), mes: (4), año: (2020) });
        let mut suscripcion1 = Suscripcion::new(TipoSuscripcion::Super, 3, Fecha { dia: (12), mes: (4), año: (2020) });

        //Test upgrade. Ok.
        assert_eq!(suscripcion0.tipo.soy_basic(), true);
        assert!(suscripcion0.upgrade().is_ok());
        assert_eq!(suscripcion0.tipo.soy_clasic(), true);

        assert!(suscripcion1.upgrade().is_err());

        //Test downgrade. Ok.
        assert!(suscripcion0.downgrade().is_ok());
        assert_eq!(suscripcion0.tipo.soy_basic(), true);
        assert!(suscripcion0.downgrade().is_err());
        assert_eq!(suscripcion0.activa, false);
    }

    #[test]
    fn test_operaciones_usuario() {

        let medio_pago = MedioPago::Efectivo;
        let mut user = Usuario::new(medio_pago, 123, "pepe".to_string(), "P".to_string(), "ape".to_string(), "email".to_string());

        let mut suscripcion = Suscripcion::new(TipoSuscripcion::Basic, 3, Fecha { dia: (12), mes: (4), año: (2020) });

        user.agregar_suscripcion(suscripcion);
        assert!(user.obtener_suscripcion_activa().is_some()); //Ok.
       
        
        
        assert!(user.upgrade_suscripcion().is_ok()); //Ok.
        assert!(user.downgrade_suscripcion().is_ok()); //Ok.
    }
    
 
    
}