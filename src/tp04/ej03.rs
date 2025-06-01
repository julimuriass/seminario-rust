use std::collections::HashMap;
use std::ptr::eq;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct Suscripcion {
    tipo: TipoSuscripcion,
    duracion_meses: u8,
    fecha_inicio: String,
    activa: bool,
}

impl Suscripcion {
    fn new(tipo: TipoSuscripcion, duracion_meses: u8, fecha_inicio: String) -> Suscripcion {
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    fn tiene_suscripcion_activa(&self) -> bool {
        self.suscripciones.iter().any(|s| s.activa)
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

#[derive(Clone, Debug)]
struct StreamingRust {
    usuarios: Vec<Usuario>, 
}

impl StreamingRust {
    fn crear_plataforma() -> StreamingRust {
        StreamingRust {
            usuarios: Vec::new(),
        }
    }

    fn crear_usuario(&mut self, suscripcion: &Suscripcion, medio_pago: &MedioPago, id: u32, username: String, nombre: String, apellido: String, email: String) {
        let usuario = Usuario {
            id: id,
            suscripciones: vec![suscripcion.clone()],
            medio_pago: medio_pago.clone(),
            username: username,
            nombre: nombre,
            apellido: apellido,
            email: email,
        };

        self.usuarios.push(usuario);
    }

    fn upgrade_suscripcion(&mut self, usuario: &mut Usuario) {
        //Given an user upgrade their subscription.
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == usuario.id) {
            user.upgrade_suscripcion(); //Update subscription.
        }
    }

    fn downgrade_suscripcion(&mut self, usuario: &mut Usuario) {
        //Given an user downgrade their subscription.
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == usuario.id) {
            user.downgrade_suscripcion(); //Downgrade subscription.
        }
    }

    fn cancelar_suscripcion(&mut self, usuario: &mut Usuario) {
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == usuario.id) {
            user.cancelar_suscripcion(); //Downgrade subscription.
        }
    }

    //Estadísticas.
    //Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones activas.
    fn suscripcion_mas_contratada_activos(&self) -> Option<TipoSuscripcion> {
        let mut aux_vec: Vec<(TipoSuscripcion, u32)> = Vec::new();

        if self.usuarios.is_empty() {
            return None;
        }

        //Ir llenando el aux_vec con las suscripciones de los usuarios activos.
        self.usuarios.iter().filter(|u| u.tiene_suscripcion_activa()) //Filtro usuarios con suscripción ctiva.
            .for_each(|u| { //Para cada una de ellas.
                //Obtener el tipo de suscripción activa.
                if let Some(suscripcion) = u.obtener_suscripcion_activa() {
                    //Si el tipo de suscripción ya existe en el vector auxiliar aumento en 1 la cantidad de veces que aparece.
                    if let Some(entry) = aux_vec.iter_mut().find(|(tipo, _)| *tipo == suscripcion.tipo) {
                        entry.0 += 1;
                    } else {  //Si no existe creo la posición con el Tipo de suscripción y un valor inicial de uno.
                        aux_vec.push((suscripcion.tipo.clone(), 1));

                    }
                }
               
            });

        aux_vec.iter()
        .max_by_key(|&(_, cantidad)| cantidad)  //Tengo que ver cuál es el máximo de mi vector auxiliar.
        .map(|(nombre, cantidad)| (nombre.clone(), *cantidad)) 
    }


}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_upgrade_downgrade() {
        let mut suscripcion0 = Suscripcion::new(TipoSuscripcion::Basic, 3, "12/9/2020".to_string());
        let mut suscripcion1 = Suscripcion::new(TipoSuscripcion::Super, 3, "4/9/2019".to_string());

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
    }
    
 
    
}