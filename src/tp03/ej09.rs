use std::collections::VecDeque;
use crate::tp03::ej03::Fecha;



#[derive(Clone)]
enum TipoAnimal {
    PERRO,
    GATO,
    CABALLO,
    OTROS,
}

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: i32,
    atenciones: VecDeque<Atencion>,
    atenciones_realizadas: Vec<Atencion>,
}

#[derive(Clone)]
struct Atencion {
    mascota: Mascota,
    diagnostico_final: String,
    tratamiento: String,
    fecha: Option<Fecha>,
}

#[derive(Clone)]
struct Mascota {
    nombre: String,
    edad: String,
    tipo: TipoAnimal,
    dueño: Dueño,
}

#[derive(Clone)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: u32,
}

pub fn compare_owners(owner1: &Dueño, owner2: &Dueño) -> bool {
    owner1.direccion == owner2.direccion &&
    owner1.nombre == owner2.nombre &&
    owner1.telefono == owner2.telefono
}

pub fn compare_type(type1: &TipoAnimal, type2: &TipoAnimal)  -> bool{
    match (type1, type2) {
        (TipoAnimal::CABALLO, TipoAnimal::CABALLO) => true,
        (TipoAnimal::PERRO, TipoAnimal::PERRO) => true,
        (TipoAnimal::GATO, TipoAnimal::GATO) => true,
        (TipoAnimal::OTROS, TipoAnimal::OTROS) => true,
        _ => false, 
    }
}

pub fn compare_pets (mascota1: &Mascota, mascota2: &Mascota) -> bool {
    compare_owners(&mascota1.dueño, &mascota2.dueño) &&
    mascota1.edad == mascota2.edad &&
    mascota1.nombre == mascota2.nombre &&
    compare_type(&mascota1.tipo, &mascota2.tipo)
}

fn compare_fecha(fecha1: &Option<Fecha>, fecha2: &Option<Fecha>) -> bool {
    match (fecha1, fecha2) {
        (Some(f1), Some(f2)) => f1.dia == f2.dia && f1.mes == f2.mes && f1.año == f2.año,
        (None, None) => true, // Ambas son None → consideradas iguales
        _ => false,           // Una tiene valor y la otra no → distintas
    }
}



pub fn compare_atencion(atencion1: &Atencion, atencion2: &Atencion) -> bool {
    atencion1.diagnostico_final == atencion2.diagnostico_final &&
    compare_fecha(&atencion1.fecha, &atencion2.fecha) &&
    atencion1.tratamiento == atencion2.tratamiento &&
    compare_pets(&atencion1.mascota, &atencion2.mascota)
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: i32) -> Veterinaria {
        Veterinaria {
            nombre,
            direccion,
            id,
            atenciones: VecDeque::new(),
            atenciones_realizadas: Vec::new(),
        }
    }

    fn agregar_nueva_mascota(&mut self, mascota: Mascota, fecha: Option<Fecha>) {
        let nueva_atencion = Atencion {
            mascota,
            diagnostico_final: String::new(),
            tratamiento: String::new(),
            fecha,
        };

        self.atenciones.push_back(nueva_atencion);
    }

    fn agregar_mascota_maxima_prioridad(&mut self, mascota: Mascota, fecha: Option<Fecha>) {
        let nueva_atencion= Atencion {
            mascota,
            diagnostico_final: String::new(),
            tratamiento: String::new(),
            fecha,
        };

        self.atenciones.push_front(nueva_atencion);
    }

    fn atender_mascota(&mut self) -> Option<Mascota>{
        if let Some(mascota_atendida)= self.atenciones.pop_front() {
            return Some(mascota_atendida.mascota);
        } 
        None
    }

    fn eliminar_mascota(&mut self, mascota: Mascota) {
        //Find pet.
        let mut indice_mascota_retirada:i32= -1;
        for i in 0..self.atenciones.len() {
            if compare_pets(&self.atenciones[i].mascota,& mascota) {
                indice_mascota_retirada= i as i32;
                break;
            }
        }
        //Delete pet.
        if indice_mascota_retirada != -1 { //If I found the pet.
            self.atenciones.remove(indice_mascota_retirada as usize);
        }
    }

    fn registrar_atencion(&mut self, tratamiento: String, diagnostico: String, fecha: Option<Fecha>) {
        if let Some(mascota_atendida)= self.atenciones.pop_front() {
            let atencion_hecha= Atencion {
                mascota: mascota_atendida.mascota,
                diagnostico_final: diagnostico,
                tratamiento: tratamiento,
                fecha,
            };

            self.atenciones_realizadas.push(atencion_hecha);
        };
    }

    fn buscar_atencion(&self, nombre_mascota: String, nombre_dueño: String, telefono: u32) -> Option<Atencion> {
        for atencion in self.atenciones.iter() {
            if atencion.mascota.nombre == nombre_mascota && atencion.mascota.dueño.nombre == nombre_dueño && atencion.mascota.dueño.telefono == telefono {
                return Some(atencion.clone());
            }
        }
        None
    }

    fn modificar_diagnostico(&mut self, diagnostico_nuevo: String, atencion: &Atencion) {
        //Buscar la atención.
        let mut indice_atencion:i32= -1;
        for i in 0..self.atenciones.len() {
            if compare_atencion(&self.atenciones[i], &atencion) {
                indice_atencion= i as i32;
                break;
            }
        }
        //Modify diagnostic.
        if indice_atencion!= -1 { //If I found the attention.
            self.atenciones[indice_atencion as usize].diagnostico_final = diagnostico_nuevo;
        }
    }

    fn modificar_fecha(&mut self, fecha_nueva: Option<Fecha>, atencion: &Atencion) {
        //Buscar la atención.
        let mut indice_atencion:i32= -1;
        for i in 0..self.atenciones.len() {
            if compare_atencion(&self.atenciones[i], &atencion) {
                indice_atencion= i as i32;
                break;
            }
        }

        if indice_atencion != -1 { 
            self.atenciones[indice_atencion as usize].fecha = fecha_nueva;
        }
    }

    fn eliminar_atencion (&mut self, atencion: &Atencion) {
        //Buscar la atención.
        let mut indice_atencion:i32= -1;
        for i in 0..self.atenciones.len() {
            if compare_atencion(&self.atenciones[i], &atencion) {
                indice_atencion= i as i32;
                break;
            }
        }

        if indice_atencion != -1 {
            self.atenciones.remove(indice_atencion as usize);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::tp03::ej03::Fecha;

    fn crear_dueño() -> Dueño {
        Dueño {
            nombre: String::from("Juan Perez"),
            direccion: String::from("Calle Falsa 123"),
            telefono: 12345678,
        }
    }

    fn crear_mascota(nombre: &str, tipo: TipoAnimal) -> Mascota {
        Mascota {
            nombre: String::from(nombre),
            edad: String::from("5"),
            tipo,
            dueño: crear_dueño(),
        }
    }

    fn crear_fecha() -> Option<Fecha> {
        Some(Fecha { dia: 1, mes: 1, año: 2024 })
    }

    #[test]
    fn test_agregar_nueva_mascota() {
        let mut vet = Veterinaria::new(String::from("Vet 1"), String::from("Arg"), 1);
        let mascota = crear_mascota("Firulais", TipoAnimal::PERRO);

        vet.agregar_nueva_mascota(mascota.clone(), crear_fecha());
        assert_eq!(vet.atenciones.len(), 1);
    }

    #[test]
    fn test_agregar_mascota_maxima_prioridad() {
        let mut vet = Veterinaria::new(String::from("Vet 2"), String::from("Arg "), 2);
        let m1 = crear_mascota("Gato", TipoAnimal::GATO);
        let m2 = crear_mascota("Caballo", TipoAnimal::CABALLO);

        vet.agregar_nueva_mascota(m1.clone(), crear_fecha());
        vet.agregar_mascota_maxima_prioridad(m2.clone(), crear_fecha());

        assert_eq!(vet.atenciones.front().unwrap().mascota.nombre, "Caballo");
    }

    #[test]
    fn test_atender_mascota() {
        let mut vet = Veterinaria::new(String::from("Vet 3"), String::from("Arg"), 3);
        let mascota = crear_mascota("Toby", TipoAnimal::GATO);

        vet.agregar_nueva_mascota(mascota.clone(), crear_fecha());
        let atendido = vet.atender_mascota();

        assert!(atendido.is_some());
        assert_eq!(atendido.unwrap().nombre, "Toby");
        assert_eq!(vet.atenciones.len(), 0);
    }

    #[test]
    fn test_registrar_atencion() {
        let mut vet = Veterinaria::new(String::from("Vet 4"), String::from("Arg"), 4);
        let mascota = crear_mascota("Rex", TipoAnimal::PERRO);
        let fecha = crear_fecha();

        vet.agregar_nueva_mascota(mascota.clone(), None);
        vet.registrar_atencion(String::from("Antibióticos"), String::from("Infección"), fecha.clone());

        assert_eq!(vet.atenciones.len(), 0);
        assert_eq!(vet.atenciones_realizadas.len(), 1);
        let atendida = &vet.atenciones_realizadas[0];
        assert_eq!(atendida.mascota.nombre, "Rex");
        assert_eq!(atendida.tratamiento, "Antibióticos");
        assert_eq!(atendida.diagnostico_final, "Infección");
        assert!(compare_fecha(&atendida.fecha, &fecha));
    }

    #[test]
    fn test_eliminar_mascota() {
        let mut vet = Veterinaria::new(String::from("Vet 5"), String::from("Zoológico"), 5);
        let mascota = crear_mascota("Milo", TipoAnimal::GATO);

        vet.agregar_nueva_mascota(mascota.clone(), None);
        assert_eq!(vet.atenciones.len(), 1);

        vet.eliminar_mascota(mascota);
        assert_eq!(vet.atenciones.len(), 0);
    }

    #[test]
    fn test_buscar_atencion() {
        let mut vet = Veterinaria::new(String::from("Vet 6"), String::from("BuscaVet"), 6);
        let mascota = crear_mascota("Rocky", TipoAnimal::PERRO);

        vet.agregar_nueva_mascota(mascota.clone(), None);
        let resultado = vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);

        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().mascota.nombre, "Rocky");
    }
}


