use std::{collections::VecDeque, path::PathBuf};
use crate::tp03::ej03::Fecha;

use serde::{Serialize, Deserialize};
use core::arch;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;


#[derive(Clone, Serialize, Deserialize)]
enum TipoAnimal {
    PERRO,
    GATO,
    CABALLO,
    OTROS,
}

#[derive(Clone, Serialize, Deserialize)]
struct Veterinaria {
    nombre: String,
    direccion: String,
    id: i32,
    atenciones: VecDeque<Atencion>,
    atenciones_realizadas: Vec<Atencion>,
    archivo_atenciones: PathBuf,
}

#[derive(Clone, Serialize, Deserialize)]
struct Atencion {
    mascota: Mascota,
    diagnostico_final: String,
    tratamiento: String,
    fecha: Option<Fecha>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Mascota {
    nombre: String,
    edad: String,
    tipo: TipoAnimal,
    dueño: Dueño,
}

#[derive(Clone, Serialize, Deserialize)]
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


enum ErroresPersonalizados {
    ErrorArchivo,
    MascotaNoEncontrada,
    AtencionNoEncontrada,
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: i32, archivo_atenciones: String) -> Veterinaria {
        let path = PathBuf::from(archivo_atenciones);
        Veterinaria {
            nombre,
            direccion,
            id,
            atenciones: VecDeque::new(),
            atenciones_realizadas: Vec::new(),
            archivo_atenciones: path,
        }
    }

    pub fn cargar_al_archivo(&mut self, atencion: &Atencion) -> Result<(), ErroresPersonalizados> {
        let mut archivo: File = match File::create(self.archivo_atenciones.clone()) {
            Err(e) => Err(ErroresPersonalizados::ErrorArchivo)?,
            Ok(arch) => arch,
        };

        let atencion_serializada = serde_json::to_string(&atencion).unwrap();
        match archivo.write(&atencion_serializada.as_bytes()) {
            Err(e) => Err(ErroresPersonalizados::ErrorArchivo)?,
            Ok(_) => Ok(()),
        }
    }

    pub fn agregar_nueva_mascota(&mut self, mascota: Mascota, fecha: Option<Fecha>) {
        let nueva_atencion = Atencion {
            mascota,
            diagnostico_final: String::new(),
            tratamiento: String::new(),
            fecha,
        };

        self.atenciones.push_back(nueva_atencion.clone());
        self.cargar_al_archivo(&nueva_atencion.clone());
    }

    pub fn cargar_al_inicio_del_archivo(&mut self) -> Result<(), ErroresPersonalizados> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.archivo_atenciones)
            .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;
    
        let writer = std::io::BufWriter::new(&file);
        serde_json::to_writer(writer, &self.atenciones)
            .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;
        
        Ok(())
    }

    fn agregar_mascota_maxima_prioridad(&mut self, mascota: Mascota, fecha: Option<Fecha>) {
        let nueva_atencion= Atencion {
            mascota,
            diagnostico_final: String::new(),
            tratamiento: String::new(),
            fecha,
        };

        self.atenciones.push_front(nueva_atencion.clone());
        //Tendría que hacer que la atención nueva también esté en el archivo al inicio (para que persista con el orden en el que se encuentran los datos en la estructura).
        self.cargar_al_inicio_del_archivo();
    }

    fn atender_mascota(&mut self) -> Option<Mascota>{
        if let Some(mascota_atendida)= self.atenciones.pop_front() {
            return Some(mascota_atendida.mascota);
            //Está bien modificar el archivo acá???
            //Tendría que eliminar a la mascota de atenciones.
            //Tendría que modificar mi atender mascota original para que lo agregue a atenciones realizadas, no?

        } 
        None
    }

    fn eliminar_mascota(&mut self, mascota: Mascota) -> Result<(), ErroresPersonalizados>{
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

            //Modify the JSON file. 
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&self.archivo_atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            let writer = std::io::BufWriter::new(&file);
            serde_json::to_writer(writer, &self.atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            Ok(())
        } else {
            return Err(ErroresPersonalizados::MascotaNoEncontrada);
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

    fn modificar_diagnostico(&mut self, diagnostico_nuevo: String, atencion: &Atencion) -> Result<(), ErroresPersonalizados>{
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
            //Modify the JSON file. 
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&self.archivo_atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            let writer = std::io::BufWriter::new(&file);
            serde_json::to_writer(writer, &self.atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            Ok(())
        } else {
            return Err(ErroresPersonalizados::MascotaNoEncontrada);
        }
    }

    fn modificar_fecha(&mut self, fecha_nueva: Option<Fecha>, atencion: &Atencion) -> Result<(), ErroresPersonalizados> {
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

            //Modify the JSON file. 
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&self.archivo_atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            let writer = std::io::BufWriter::new(&file);
            serde_json::to_writer(writer, &self.atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            Ok(())
        } else {
            return Err(ErroresPersonalizados::MascotaNoEncontrada);
        }
    }

    fn eliminar_atencion (&mut self, atencion: &Atencion) -> Result<(), ErroresPersonalizados> {
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
            //Modify the JSON file. 
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&self.archivo_atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            let writer = std::io::BufWriter::new(&file);
            serde_json::to_writer(writer, &self.atenciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

            Ok(())
        } else {
            return Err(ErroresPersonalizados::MascotaNoEncontrada);
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
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 1"), String::from("Arg"), 1, String::from(path));
        let mascota = crear_mascota("Firulais", TipoAnimal::PERRO);

        vet.agregar_nueva_mascota(mascota.clone(), crear_fecha());
        assert_eq!(vet.atenciones.len(), 1);
    }

    #[test]
    fn test_agregar_mascota_maxima_prioridad() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 2"), String::from("Arg "), 2, String::from(path));
        let m1 = crear_mascota("Gato", TipoAnimal::GATO);
        let m2 = crear_mascota("Caballo", TipoAnimal::CABALLO);

        vet.agregar_nueva_mascota(m1.clone(), crear_fecha());
        vet.agregar_mascota_maxima_prioridad(m2.clone(), crear_fecha());

        assert_eq!(vet.atenciones.front().unwrap().mascota.nombre, "Caballo");
    }

    #[test]
    fn test_atender_mascota() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 3"), String::from("Arg"), 3, String::from(path));
        let mascota = crear_mascota("Toby", TipoAnimal::GATO);

        vet.agregar_nueva_mascota(mascota.clone(), crear_fecha());
        let atendido = vet.atender_mascota();

        assert!(atendido.is_some());
        assert_eq!(atendido.unwrap().nombre, "Toby");
        assert_eq!(vet.atenciones.len(), 0);
    }

    #[test]
    fn test_registrar_atencion() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 4"), String::from("Arg"), 4 , String::from(path));
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
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 5"), String::from("Zoológico"), 5, String::from(path));
        let mascota = crear_mascota("Milo", TipoAnimal::GATO);

        vet.agregar_nueva_mascota(mascota.clone(), None);
        assert_eq!(vet.atenciones.len(), 1);

        vet.eliminar_mascota(mascota);
        assert_eq!(vet.atenciones.len(), 0);
    }

    #[test]
    fn test_buscar_atencion() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 6"), String::from("BuscaVet"), 6, String::from(path));
        let mascota = crear_mascota("Rocky", TipoAnimal::PERRO);

        vet.agregar_nueva_mascota(mascota.clone(), None);
        let resultado = vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);

        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().mascota.nombre, "Rocky");
    }

    #[test]
    fn test_modificar_diagnostico() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 6"), String::from("BuscaVet"), 6, String::from(path));
        let mascota = crear_mascota("Rocky", TipoAnimal::PERRO);

        vet.agregar_nueva_mascota(mascota.clone(), None);
        let mut atencion = vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);

        assert!(atencion.is_some());
        vet.modificar_diagnostico("Fiebre".to_string(), &atencion.unwrap());

        let updated_atencion =  vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);
        assert!(updated_atencion.is_some());
        atencion = Some(updated_atencion.unwrap());

        assert_eq!(atencion.unwrap().diagnostico_final, String::from("Fiebre")); //Ok.
    }

    #[test]
    fn test_modificar_fecha() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 6"), String::from("BuscaVet"), 6, String::from(path));
        let mascota = crear_mascota("Rocky", TipoAnimal::PERRO);
        let mut fecha = crear_fecha();

        vet.agregar_nueva_mascota(mascota.clone(), fecha);
        let mut atencion = vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);

        let fecha_nueva = Fecha {
            año: 2025,
            dia: 24,
            mes: 12,
        };
        vet.modificar_fecha(Some(fecha_nueva), &atencion.unwrap());

        let updated_atencion =  vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);
        assert!(updated_atencion.is_some());
        atencion = Some(updated_atencion.unwrap());

        assert_eq!(atencion.clone().unwrap().fecha.unwrap().año, 2025);
        assert_eq!(atencion.clone().unwrap().fecha.unwrap().mes, 12);
        assert_eq!(atencion.clone().unwrap().fecha.unwrap().dia, 24);
    }

    #[test]
    fn test_eliminar_atencion() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 6"), String::from("BuscaVet"), 6, String::from(path));
        let mascota = crear_mascota("Rocky", TipoAnimal::PERRO);
        let mut fecha = crear_fecha();

        vet.agregar_nueva_mascota(mascota.clone(), fecha);
        let mut atencion = vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);

        assert_eq!(vet.atenciones.len(), 1); //Estado inicial.

        vet.eliminar_atencion(&atencion.unwrap());
        assert_eq!(vet.atenciones.len(), 0); //Ok.
    }

    #[test]
    fn test_agregar_mascota_maxima_prioridad_archivo() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 2"), String::from("Arg "), 2, String::from(path));
        let m1 = crear_mascota("Gato", TipoAnimal::GATO);
        let m2 = crear_mascota("Caballo", TipoAnimal::CABALLO);

        vet.agregar_nueva_mascota(m1.clone(), crear_fecha());
        vet.agregar_mascota_maxima_prioridad(m2.clone(), crear_fecha());

        assert_eq!(vet.atenciones.front().unwrap().mascota.nombre, "Caballo");
        //En el archivo aparece Caballo primero!. Ok.
    }

    #[test]
    fn test_eliminar_mascota_nuevo() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 5"), String::from("Zoológico"), 5, String::from(path));
        let mascota = crear_mascota("Milo", TipoAnimal::GATO);

        vet.agregar_nueva_mascota(mascota.clone(), None);
        assert_eq!(vet.atenciones.len(), 1);

        assert!(vet.eliminar_mascota(mascota).is_ok());
        assert_eq!(vet.atenciones.len(), 0);

        let mascota2 = crear_mascota("Pepe", TipoAnimal::GATO);
        assert!(vet.eliminar_mascota(mascota2.clone()).is_err());

    }

    #[test]
    fn eliminar_atencion_nuevo() {
        let path = "src/tp05/archivo_atenciones.txt";
        let mut vet = Veterinaria::new(String::from("Vet 6"), String::from("BuscaVet"), 6, String::from(path));
        let mascota = crear_mascota("Rocky", TipoAnimal::PERRO);
        let mut fecha = crear_fecha();
        let mascota2 = crear_mascota("Pepe", TipoAnimal::PERRO);

        vet.agregar_nueva_mascota(mascota.clone(), fecha);
        let mut atencion = vet.buscar_atencion(String::from("Rocky"), String::from("Juan Perez"), 12345678);

        assert_eq!(vet.atenciones.len(), 1); //Estado inicial.

        assert!(vet.eliminar_atencion(&atencion.unwrap()).is_ok());
        assert_eq!(vet.atenciones.len(), 0); //Ok.

        
        /*let mut atencion2= Atencion { mascota2.clone(), String::from("fiebre"), String::from("antibioticos"), None}; //Por qué no me deja???
        assert!(vet.eliminar_atencion(&atencion2).is_err());*/

    }
}


