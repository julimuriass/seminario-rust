struct Examen {
    nombre_materia: String,
    nota: f64,
}

struct Estudiante {
    nombre: String,
    id: i32,
    examenes: Vec<Examen>,

}

impl Examen {
    fn new(nombre_materia: String, nota: f64) -> Examen {
        Examen {
            nombre_materia,
            nota,
        }
    }
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
}

#[cfg(test)]
mod  test {
    use super::*;

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

