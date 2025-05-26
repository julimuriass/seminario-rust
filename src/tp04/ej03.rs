use crate::tp03::ej03::Fecha;

enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

struct Suscripcion {
    tipo: TipoSuscripcion,
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: Fecha,
    activa: bool,
}

impl Suscripcion {
    fn downgrade (&mut self) {
        match self.tipo {
            TipoSuscripcion::Basic => {self.activa = false;}
            TipoSuscripcion::Clasic => {self.tipo = TipoSuscripcion::Basic;}
            TipoSuscripcion::Super => {self.tipo = TipoSuscripcion::Clasic;}
        }
    }

    fn upgrade (&mut self) {
        match self.tipo {
            TipoSuscripcion::Basic => {self.tipo = TipoSuscripcion::Clasic;}
            TipoSuscripcion::Clasic => {self.tipo = TipoSuscripcion::Super;}
            _ => {}
        }
    }

    fn cancelarse (&mut self) {
        self.activa = false
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_downgrade() {
        let mut suscripcion = Suscripcion{tipo: TipoSuscripcion::Basic, 
                                                    costo_mensual: 100.0, duracion_meses: 1,
                                                    fecha_inicio: Fecha {dia: (12), mes: (3), año: (2000)},
                                                    activa: true};
        
        assert_eq!(suscripcion.activa, true);
        suscripcion.downgrade();
        assert_eq!(suscripcion.activa, false); //From basic to cancelled. Ok.

        //From super to clasic.
        let mut suscripcion1 = Suscripcion{tipo: TipoSuscripcion::Super, 
            costo_mensual: 100.0, duracion_meses: 1,
            fecha_inicio: Fecha {dia: (12), mes: (3), año: (2000)},
            activa: true};
        
        let is_super = match suscripcion1.tipo {
            TipoSuscripcion::Super => true,
            _ => false,
        };

        assert!(is_super);

        suscripcion1.downgrade();
        assert_eq!(suscripcion1.activa, true);
        
        let is_clasic = match suscripcion1.tipo {
            TipoSuscripcion::Clasic => true,
            _ => false,
        };
        assert_eq!(is_clasic, true);
    }

    #[test]
    fn test_upgrade() {
        //From basic to clasic.
        let mut suscripcion = Suscripcion{tipo: TipoSuscripcion::Basic, 
            costo_mensual: 100.0, duracion_meses: 1,
            fecha_inicio: Fecha {dia: (12), mes: (3), año: (2000)},
            activa: true};
        
        suscripcion.upgrade();
        let is_clasic = match suscripcion.tipo {
            TipoSuscripcion::Clasic => true,
            _ => false,
        };
        assert_eq!(is_clasic, true);

        //From super to super (nothing changes).
        let mut suscripcion1 = Suscripcion{tipo: TipoSuscripcion::Super, 
            costo_mensual: 100.0, duracion_meses: 1,
            fecha_inicio: Fecha {dia: (12), mes: (3), año: (2000)},
            activa: true};
        
        suscripcion1.upgrade();
        let is_super = match suscripcion1.tipo {
            TipoSuscripcion::Super => true,
            _ => false,
        };
        assert_eq!(is_super, true);

    }

    #[test]
    fn test_cancelarse() {
        let mut suscripcion = Suscripcion{tipo: TipoSuscripcion::Basic, 
            costo_mensual: 100.0, duracion_meses: 1,
            fecha_inicio: Fecha {dia: (12), mes: (3), año: (2000)},
            activa: true};

        assert!(suscripcion.activa);

        suscripcion.cancelarse();
        assert_eq!(suscripcion.activa, false); //Test Ok. A suscription knows how to cancel itself.
    }
}