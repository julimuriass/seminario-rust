struct Examen {
    nombre_materia: String,
    nota: f64,
}

struct Estudiante {
    nombre: String,
    id: i32,
    examenes: Vec<Examen>,

}

struct Informe {
    nombre: String,
    id: i32,
    cant_examenes_rendidos: u32,
    promedio: f64,
    nota_baja: f64,
    materia_baja: String,
    nota_alta: f64,
    materia_alta: String,
}

impl Examen {
    fn new(nombre_materia: String, nota: f64) -> Examen {
        Examen {
            nombre_materia,
            nota,
        }
    }
}

pub fn obtener_materia_calificacion_alta(examenes: &Vec<Examen>) -> String {
    //función auxiliar que busca la nota más alta y se queda con la materia asociada a ésta.
    if examenes.len() == 0 {
        return String::from("No hay.");
    }

    let mut nota_mas_alta = f64::MIN;
    let mut materia= String::from("materia");
    
    for examen in examenes.iter() {
        if examen.nota > nota_mas_alta {
            nota_mas_alta= examen.nota;
            materia= examen.nombre_materia.clone();

        }
    }

    materia
}

pub fn obtener_materia_calificacion_baja(examenes: &Vec<Examen>) -> String {
    //función auxiliar que busca la nota más baja y se queda con la materia asociada a ésta.
    if examenes.len() == 0 {
        return String::from("No hay");
    }

    let mut nota_mas_baja = f64::MAX;
    let mut materia= String::from("materia");
    
    for examen in examenes.iter() {
        if examen.nota < nota_mas_baja {
            nota_mas_baja= examen.nota;
            materia= examen.nombre_materia.clone();
        }
    }

    materia
}


impl Estudiante {
    fn new(nombre: String, id: i32) -> Estudiante {
        Estudiante {
            nombre,
            id,
            examenes: Vec::new(),
        }
    }

    fn obtener_promedio(&self) -> f64 {
        let total_notas= self.examenes.len();
        
        if total_notas > 0 {
            let mut suma_notas= 0.0;
            for examen in self.examenes.iter(){
                suma_notas+= examen.nota;
            }
            return suma_notas/total_notas as f64;
        }

        0.0
         
     }

     fn obtener_calificacion_mas_alta(&self) -> f64 {
        if self.examenes.len() == 0 {
            return 0.0;
        }

        let mut nota_mas_alta = f64::MIN;
        
        for examen in self.examenes.iter() {
            if examen.nota > nota_mas_alta {
                nota_mas_alta= examen.nota;
            }
        }

        nota_mas_alta
     }

     fn obtener_calificacion_mas_baja(&self) -> f64 {
        if self.examenes.len() == 0 {
            return 0.0;
        }

        let mut nota_mas_baja = f64::MAX;
        
        for examen in self.examenes.iter() {
            if examen.nota < nota_mas_baja {
                nota_mas_baja= examen.nota;
            }
        }

        nota_mas_baja
     }

     fn generar_informe (&self) -> Option<Informe> {
        if self.examenes.len() == 0 {
            return None; //Si no tiene notas devuelvo nada.
        } else {
            let informe = Informe { //Creo el informe y le aigno los campos.
                nombre : self.nombre.clone(), 
                id: self.id,
                cant_examenes_rendidos: self.examenes.len() as u32 -1,
                promedio: self.obtener_promedio(),
                nota_alta: self.obtener_calificacion_mas_alta(),
                nota_baja: self.obtener_calificacion_mas_baja(),
                materia_baja: obtener_materia_calificacion_baja(&self.examenes),
                materia_alta: obtener_materia_calificacion_alta(&self.examenes),
            };

            return Some(informe);
        }

     }

     

}

#[cfg(test)]
mod  test {
    use super::*;


    #[test]
    fn test_materias_asociadas_a_notas() {
        let mut student= Estudiante::new(String::from("Julieta"), 123);
        student.examenes.push(Examen::new(String::from("cadp"), 8.0));
        student.examenes.push(Examen::new(String::from("inglés"), 10.0));

        let materia1= obtener_materia_calificacion_alta(&student.examenes);
        assert_eq!(materia1, String::from("inglés"));

        let materia2 = obtener_materia_calificacion_baja(&student.examenes);
        assert_eq!(materia2, String::from("cadp"));

    }

    #[test]
    fn test_entregable1() {
        let mut student= Estudiante::new(String::from("Julieta"), 123);
        student.examenes.push(Examen::new(String::from("cadp"), 8.0));
        student.examenes.push(Examen::new(String::from("inglés"), 10.0));

        let informe1 = student.generar_informe();
        assert!(informe1.is_some()); //pruebo con un estudiante que sí tenga informe.


        let mut student= Estudiante::new(String::from("Pepe"), 123);
        let informe2 = student.generar_informe();
        assert!(informe2.is_none());

    }


    #[test]
    fn test_promedio() {
        let mut student= Estudiante::new(String::from("Julieta"), 123);
        student.examenes.push(Examen::new(String::from("cadp"), 8.0));
        student.examenes.push(Examen::new(String::from("inglés"), 10.0));

        assert_eq!(student.obtener_promedio(), 9.0);

        let mut student2= Estudiante::new(String::from("Pepe"), 999);
        assert_eq!(student2.obtener_promedio(), 0.0);
    }

    #[test]
    fn test_obtener_calificacion_alta() {
        let mut student= Estudiante::new(String::from("Julieta"), 123);
        student.examenes.push(Examen::new(String::from("cadp"), 8.0));
        student.examenes.push(Examen::new(String::from("inglés"), 10.0));

        assert_eq!(student.obtener_calificacion_mas_alta(), 10.0);

        let mut student2= Estudiante::new(String::from("Pepe"), 999);
        assert_eq!(student2.obtener_calificacion_mas_alta(), 0.0);
    }

    #[test]
    fn test_obtener_calificacion_baja() {
        let mut student= Estudiante::new(String::from("Julieta"), 123);
        student.examenes.push(Examen::new(String::from("cadp"), 8.0));
        student.examenes.push(Examen::new(String::from("inglés"), 10.0));

        assert_eq!(student.obtener_calificacion_mas_baja(), 8.0);

        let mut student2= Estudiante::new(String::from("Pepe"), 999);
        assert_eq!(student2.obtener_calificacion_mas_baja(), 0.0);
    }
}

