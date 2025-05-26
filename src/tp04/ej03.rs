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
            None => Err("No se puede ascender de Super.".to_string())  
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

    
 
    
}