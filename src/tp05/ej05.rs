use std::ptr::eq;

#[derive(Clone, Debug)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

impl PartialEq for TipoSuscripcion {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TipoSuscripcion::Basic, TipoSuscripcion::Basic) => true,
            (TipoSuscripcion::Clasic, TipoSuscripcion::Clasic) => true,
            (TipoSuscripcion::Super, TipoSuscripcion::Super) => true,
            _ => false,
        }
    }
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

impl MedioPago {
    fn to_string(&self) -> String {
        match self {
            MedioPago::Efectivo => String::from("Efectivo"),
            MedioPago::TarjetaCredito { numero_tarjeta } => String::from("TarjetaCredito"),
            MedioPago::MercadoPago { cbu } => String::from("MercadoPago"),
            MedioPago::TransferenciaBancaria { cuenta_destino, cuenta_origen } => String::from("TransferenciaBancaria"),
            MedioPago::Cripto { tipo_cripto } => String::from("Cripto"),
        }
    }
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

    pub fn crear_plataforma() -> StreamingRust {
        StreamingRust {
            usuarios: Vec::new(),
        }
    }

    pub fn crear_usuario(&mut self, suscripcion: &Suscripcion, medio_pago: &MedioPago, id: u32, username: String, nombre: String, apellido: String, email: String) {
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

    pub fn upgrade_suscripcion(&mut self, usuario: &mut Usuario) {
        //Given an user upgrade their subscription.
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == usuario.id) {
            user.upgrade_suscripcion(); //Update subscription.
        }
    }

    pub fn downgrade_suscripcion(&mut self, usuario: &mut Usuario) {
        //Given an user downgrade their subscription.
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == usuario.id) {
            user.downgrade_suscripcion(); //Downgrade subscription.
        }
    }

    pub fn cancelar_suscripcion(&mut self, usuario: &mut Usuario) {
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == usuario.id) {
            user.cancelar_suscripcion(); //Downgrade subscription.
        }
    }

    //Estadísticas.
    pub fn suscripcion_mas_contratada_activos(&self) -> Option<(TipoSuscripcion, u32)> {
        let mut aux_vec: Vec<(TipoSuscripcion, u32)> = Vec::new();

        if self.usuarios.is_empty() {
            return None;
        }

        //Ir llenando el aux_vec con las suscripciones activas de los usuarios.
        self.usuarios.iter().filter(|u| u.tiene_suscripcion_activa()) //Filtro usuarios con suscripción activa.
            .for_each(|u| { //Para cada una de ellas.
                //Obtener el tipo de suscripción activa.
                if let Some(suscripcion) = u.obtener_suscripcion_activa() {
                    //Si el tipo de suscripción ya existe en el vector auxiliar aumento en 1 la cantidad de veces que aparece.
                    if let Some(entry) = aux_vec.iter_mut().find(|(tipo, _)| *tipo == suscripcion.tipo) {
                        entry.1 += 1;
                    } else {  //Si no existe creo la posición con el Tipo de suscripción y un valor inicial de uno.
                        aux_vec.push((suscripcion.tipo.clone(), 1));

                    }
                }
               
            });

        aux_vec.iter()
        .max_by_key(|&(_, cantidad)| cantidad)  //Tengo que ver cuál es el máximo de mi vector auxiliar.
        .map(|(nombre, cantidad)| (nombre.clone(), *cantidad)) 
    }

    pub fn suscripcion_mas_contratada(&self) -> Option<(TipoSuscripcion, u32)> {
        //No me importan solo las suscripciones activas, tengo que chequear todas.
        let mut aux_vec: Vec<(TipoSuscripcion, u32)> = Vec::new();

        if self.usuarios.is_empty() {
            return None;
        }

        //Ir llenando el aux_vec con las suscripciones de los usuarios.
        self.usuarios.iter()
            .for_each(|u| { //Para cada usuario.
                //Recorrer sus suscripciones (todas).
                u.suscripciones.iter().for_each(|suscripcion| {
                    //Si el tipo de suscripción ya existe en el vector auxiliar aumento en 1 la cantidad de veces que aparece.
                    if let Some(entry) = aux_vec.iter_mut().find(|(tipo, _)| *tipo == suscripcion.tipo) {
                        entry.1 += 1;
                    } else {
                        //Si no existe creo la posición con el Tipo de suscripción y un valor inicial de uno.
                        aux_vec.push((suscripcion.tipo.clone(), 1));
                    }
                });                     
            });

        aux_vec.iter()
        .max_by_key(|&(_, cantidad)| cantidad)  //Tengo que ver cuál es el máximo de mi vector auxiliar.
        .map(|(nombre, cantidad)| (nombre.clone(), *cantidad)) 
    }

    pub fn medio_pago_mas_usado_activos(&self) -> Option<(String, u32)> {
        let mut auxiliar_vec: Vec<(String, u32)> = Vec::new();

        if self.usuarios.is_empty() {
            return None;
        }
        
        //Si no coincide con ninguna, creo la posición con el medio de pago y la cantidad inicializada en 1.
        self.usuarios.iter().filter(|u| u.tiene_suscripcion_activa()) //Filtro los usuarios con suscripciones activas.
            .for_each(|u| {
                //Busco si su medio de pago coincide con alguna entrada en el vector -> aumento en 1 la cantidad.
                if let Some(entry) = auxiliar_vec.iter_mut().find(|(medio,_)| *medio == u.medio_pago.to_string()) {
                    entry.1 += 1;
                } else {
                    auxiliar_vec.push((u.medio_pago.to_string(), 1));
                }
            });

        auxiliar_vec.iter() //Creates an iterator over the elements of auxiliar_vec
        .max_by_key(|&(_, cantidad)| cantidad) //The &(_, cantidad) pattern destructures each tuple in the vector, ignoring the first element (medio) and borrowing the second element (cantidad). The closure returns the cantidad value, which is used as the key for comparison.
        .map(|(medio, cantidad)| (medio.clone(), *cantidad)) //applies the map method to transform the result of max_by_key. If max_by_key returns Some((medio, cantidad)), the closure |(medio, cantidad)| (medio.clone(), *cantidad) is applied to the tuple.
    }

    pub fn medio_pago_mas_usado(&self) -> Option<(String, u32)> {
        let mut auxiliar_vec: Vec<(String, u32)> = Vec::new();

        if self.usuarios.is_empty() {
            return None;
        }

        //Ir llenando el aux_vec con las suscripciones de los usuarios.
        self.usuarios.iter()
            .for_each(|u| { //Para cada usuario. (No filter).
                u.suscripciones.iter().for_each(|suscripcion| {
                    //Si el tipo de suscripción ya existe en el vector auxiliar aumento en 1 la cantidad de veces que aparece.
                    if let Some(entry) = auxiliar_vec.iter_mut().find(|(medio,_)| *medio == u.medio_pago.to_string()) {
                        entry.1 += 1;
                    } else {
                        //Si no existe creo la posición con el Tipo de suscripción y un valor inicial de uno.
                        auxiliar_vec.push((u.medio_pago.to_string(), 1));
                    }
                });                     
            });
        
        auxiliar_vec.iter() //Creates an iterator over the elements of auxiliar_vec
        .max_by_key(|&(_, cantidad)| cantidad) //The &(_, cantidad) pattern destructures each tuple in the vector, ignoring the first element (medio) and borrowing the second element (cantidad). The closure returns the cantidad value, which is used as the key for comparison.
        .map(|(medio, cantidad)| (medio.clone(), *cantidad)) //applies the map method to transform the result of max_by_key. If max_by_key returns Some((medio, cantidad)), the closure |(medio, cantidad)| (medio.clone(), *cantidad) is applied to the tuple.    
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
    fn test_upgrade_subscription() {
        let mut user0 = Usuario {
            username: "sus0".to_string(),
            email: "sus0@email".to_string(),
            apellido: "0".to_string(),
            id: 1,
            nombre: "sus0".to_string(),
            medio_pago: MedioPago::Efectivo,
            suscripciones: vec![Suscripcion {
                tipo: TipoSuscripcion::Basic,
                duracion_meses: 8,
                fecha_inicio: "1/1/2025".to_string(),
                activa: true,
            }],
        };

        let mut usuarios = vec![user0.clone()];
        let mut plataforma = StreamingRust {usuarios};

        plataforma.upgrade_suscripcion(&mut user0);

        // Synchronize user0 with the updated user.
        if let Some(updated_user) = plataforma.usuarios.iter().find(|u| u.id == user0.id) {
            user0.suscripciones = updated_user.suscripciones.clone();
        }

        assert!(user0.suscripciones.iter().any(|s| s.tipo == TipoSuscripcion::Clasic)); //Ok.



        let mut user1 = Usuario {
            username: "sus1".to_string(),
            email: "sus1@email".to_string(),
            apellido: "1".to_string(),
            id: 2,
            nombre: "sus1".to_string(),
            medio_pago: MedioPago::MercadoPago { cbu: 124 },
            suscripciones: vec![Suscripcion {
                tipo: TipoSuscripcion::Super,
                duracion_meses: 8,
                fecha_inicio: "1/1/2025".to_string(),
                activa: true,
            }],
        };

        plataforma.usuarios.push(user1.clone());
        plataforma.upgrade_suscripcion(&mut user1);
        assert!(user1.suscripciones.iter().any(|s| s.tipo == TipoSuscripcion::Super)); //Ok.


        plataforma.cancelar_suscripcion(&mut user0);
        // Synchronize user0 with the updated user.
        if let Some(updated_user) = plataforma.usuarios.iter().find(|u| u.id == user0.id) {
            user0.suscripciones = updated_user.suscripciones.clone();
        }
        assert_eq!(user0.suscripciones.iter().any(|s| s.activa), false); //Ok.
    }

    #[test]
    fn test_estadisticas() {
        //Creo los usuarios para agregarlos a mi plataforma después.
        let mut user0 = Usuario {
            username: "sus0".to_string(),
            email: "sus0@email".to_string(),
            apellido: "0".to_string(),
            id: 1,
            nombre: "sus0".to_string(),
            medio_pago: MedioPago::Efectivo,
            suscripciones: vec![Suscripcion {
                tipo: TipoSuscripcion::Basic,
                duracion_meses: 8,
                fecha_inicio: "1/1/2025".to_string(),
                activa: true,
            }],
        };

        let mut user1 = Usuario {
            username: "sus1".to_string(),
            email: "sus1@email".to_string(),
            apellido: "1".to_string(),
            id: 2,
            nombre: "sus1".to_string(),
            medio_pago: MedioPago::Efectivo,
            suscripciones: vec![Suscripcion {
                tipo: TipoSuscripcion::Basic,
                duracion_meses: 8,
                fecha_inicio: "1/1/2025".to_string(),
                activa: true,
            }],
        };

        let mut user2 = Usuario {
            username: "sus2".to_string(),
            email: "sus2@email".to_string(),
            apellido: "2".to_string(),
            id: 3,
            nombre: "sus2".to_string(),
            medio_pago: MedioPago::MercadoPago { cbu: 123 },
            suscripciones: vec![Suscripcion {
                tipo: TipoSuscripcion::Clasic,
                duracion_meses: 8,
                fecha_inicio: "1/1/2025".to_string(),
                activa: true,
            }],
        };

        let mut user3 = Usuario {
            username: "sus3".to_string(),
            email: "sus3@email".to_string(),
            apellido: "3".to_string(),
            id: 4,
            nombre: "sus3".to_string(),
            medio_pago: MedioPago::Efectivo,
            suscripciones: vec![Suscripcion {
                tipo: TipoSuscripcion::Clasic,
                duracion_meses: 8,
                fecha_inicio: "1/1/2025".to_string(),
                activa: false,
            }],
        };

        //En resumen: 
        //Medios de pago: Activos: 2 de efectivo, 1 de mercado pago. Inactivos: 1 Efectivo.
        //Suscripciones: Activos: 2 Basic, 1 Clasic. Inactivos: 1 Clasic.

        let mut usuarios = vec![user0.clone(), user1.clone(), user2.clone(), user3.clone()];
        let mut plataforma = StreamingRust {usuarios};

        assert_eq!(plataforma.medio_pago_mas_usado_activos(), Some(("Efectivo".to_string(), 2))); //Ok.
        assert_eq!(plataforma.medio_pago_mas_usado(), Some(("Efectivo".to_string(), 3))); //Ok.

        assert_eq!(plataforma.suscripcion_mas_contratada(), Some((TipoSuscripcion::Clasic, 2))); //Ok. (Se queda con el último valor que encontró (el clasic en este caso) si hay un empate).
        assert_eq!(plataforma.suscripcion_mas_contratada_activos(), Some((TipoSuscripcion::Basic, 2))); //Ok.
    }

    //Tests agregados: 
    #[test]
    fn test_upgrade_downgrade_2() {
        let mut suscripcion0 = Suscripcion::new(TipoSuscripcion::Clasic, 3, "12/9/2020".to_string());
        let mut suscripcion1 = Suscripcion::new(TipoSuscripcion::Super, 3, "4/9/2019".to_string());

        //Test upgrade. Ok.
        assert_eq!(suscripcion0.tipo.soy_clasic(), true);
        assert!(suscripcion0.upgrade().is_ok());
        assert_eq!(suscripcion0.tipo.soy_super(), true);

        assert!(suscripcion1.upgrade().is_err());

        //Test downgrade. Ok.
        assert_eq!(suscripcion1.tipo.soy_super(), true);
        assert!(suscripcion1.downgrade().is_ok());
        assert_eq!(suscripcion1.tipo.soy_clasic(), true);
    }

    #[test]
    fn crear_plataforma() {
        let mut plataforma:StreamingRust = StreamingRust::crear_plataforma();
        assert!(plataforma.usuarios.is_empty()); //Ok. Se crea con el vector de usuarios vacío.

        let mut suscripcion = Suscripcion::new(TipoSuscripcion::Clasic, 3, "12/9/2020".to_string());

        let mut user = plataforma.crear_usuario(&suscripcion, &MedioPago::Cripto { tipo_cripto: "Bitcoin".to_string() }, 123, "username".to_string(), "nombre".to_string(), "apellido".to_string(), "email".to_string());
        
        assert_ne!(plataforma.usuarios.is_empty(), true); //Ok.
        assert_eq!(plataforma.usuarios.len(), 1); //Ok.
    } 

}