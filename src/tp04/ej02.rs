#[derive(Clone)]
struct Persona <'a>{
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}


//Preg si está bien el uso del trait persona???
trait PersonaOps <'a> {
    fn filtrar_por_salario(&self, personas: &'a Vec<Persona<'a>>, salario: f64) -> Option<Vec<Persona<'a>>>;
    fn filtrar_por_edad_ciudad(&self, personas: &'a Vec<Persona<'a>>, edad: u8, ciudad: String) -> Option<Vec<Persona<'a>>>;
    fn todos_viven_en(&self, personas: &'a Vec<Persona<'a>>, ciudad: &str) -> bool;
    fn alguno_vive_en(&self, personas: &'a Vec<Persona<'a>>, ciudad:  &str) -> bool;
    fn persona_existe(&self, personas: &'a Vec<Persona<'a>>, persona: &Persona<'a>) -> bool;
    fn recolectar_edades(&self, personas: &'a Vec<Persona<'a>>) -> Option<Vec<u8>>;
    fn salario_mayor_menor(&self, personas: &'a Vec<Persona<'a>>) ->  Option<(&Persona<'a>, &Persona<'a>)>;
}



impl <'a> PersonaOps <'a> for Persona <'a> {

    //Está bien??? Me da muchas dudas la parte de self pq no lo uso nunca 
    
    fn filtrar_por_salario<'b> (&self, personas: &'a Vec<Persona<'b>>, salario: f64) -> Option<Vec<Persona<'b>>> {
        let people:Vec<Persona> = personas.iter().filter(|p| p.salario > salario).cloned().collect();

        if people.len() == 0 {
            return None;
        } else {
            return Some(people);
        }
    }

    fn filtrar_por_edad_ciudad<'c> (&self, personas: &'c Vec<Persona<'c>>, edad: u8, ciudad: String) -> Option<Vec<Persona<'c>>> {
        let people:Vec<Persona> = personas.iter().filter(|p| p.edad > edad && p.ciudad == ciudad).cloned().collect();

        if people.len() == 0 {
            return None;
        } else {
            return Some(people);
        }
    }

    fn todos_viven_en<'d> (&self, personas: &'d Vec<Persona<'d>>, ciudad: &str) -> bool {
        if personas.is_empty() {
            return false;
        }
        personas.iter().all(|p| p.ciudad.eq_ignore_ascii_case(&ciudad))
    }

    fn alguno_vive_en (&self, personas: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
        if personas.is_empty() {
            return false;
        }
        personas.iter().any(|p| p.ciudad.eq_ignore_ascii_case(&ciudad))
    }

    fn persona_existe<'e>(&self, personas: &'a Vec<Persona<'a>>, persona: &Persona<'e>) -> bool {
        if personas.is_empty() {
            return false;
        }
        personas.iter().any(|p| compare(p.clone(), persona.clone()))
    }

    fn recolectar_edades(&self, personas: &'a Vec<Persona<'a>>) -> Option<Vec<u8>> {
        if personas.is_empty() {
            return None;
        } else {
            let edades: Vec<u8> = personas.iter().map(|p| p.edad).collect();
            return Some(edades);
        }
    } 

    fn salario_mayor_menor(&self, personas: &'a Vec<Persona<'a>>) -> Option<(&Persona<'a>, &Persona<'a>)>{ //Return a tuple with both people.
        if personas.is_empty() {
            return None;
        } else {
            let min = personas.iter().min_by(|a, b| {  //a and b are references to the items in the vector.
                a.salario 
                    .partial_cmp(&b.salario) //Primary comparison.
                    .unwrap() //Unwrap because .partial_cmp() returns an Option, and I'm sure that there are no Nan values in the data.
                    .then(b.edad.cmp(&a.edad))  //Tie break. (Second condition; older wins!).
            });
            
            //Same but for max.
            let max = personas.iter().max_by(|a, b| {
                a.salario 
                    .partial_cmp(&b.salario)
                    .unwrap()
                    .then(b.edad.cmp(&a.edad)) 
            });

            if let (Some(min), Some(max)) = (min, max) {
                Some((min, max))
            } else { //Esto está de más????
                None
            }
        }
    }

    
}

pub fn compare (person1: Persona, person2: Persona) -> bool {
    person1.nombre == person2.nombre &&
    person1.apellido == person2.apellido &&
    person1.direccion.eq_ignore_ascii_case(person2.direccion) &&
    person1.ciudad.eq_ignore_ascii_case(person2.ciudad) &&
    person1.edad == person2.edad &&
    person1.salario == person2.salario
}

#[cfg(test)]
mod test {
    use super::*;

    fn crear_vector_con_personas<'a>() -> Vec<Persona<'a>> {
        vec![
            Persona {
                nombre: "Juli",
                apellido: "M",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 19,
                salario: 10000.0,
            },
            Persona {
                nombre: "Pepe",
                apellido: "P",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 20,
                salario: 10000.0,
            },
            Persona {
                nombre: "Nombre",
                apellido: "N",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 60,
                salario: 50000.0,
            },
            Persona {
                nombre: "Jorgito",
                apellido: "J",
                direccion: "Arg",
                ciudad: "Bs As",
                edad: 35,
                salario: 45000.0,
            },
            Persona {
                nombre: "Benja",
                apellido: "M",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 25,
                salario: 60000.0,
            },
        ]
    }


    #[test]
    fn test_filtrar_por_salario() {
        let personas= crear_vector_con_personas();
        let personas_inexistentes:Vec<Persona> = vec![];

        let persona= Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};
        let filtrados = persona.filtrar_por_salario(&personas, 10000.0);
        //There should be 3 people in filtrados.
        
        assert!(filtrados.is_some());
        assert_eq!(filtrados.as_ref().unwrap().len(), 3);
        assert_ne!(filtrados.as_ref().unwrap().len(), 5);


        //Test with the empty vec.
        let filtrados_vacios = persona.filtrar_por_salario(&personas_inexistentes, 10000.0);
        assert!(filtrados_vacios.is_none());
    }

    #[test]
    fn test_filtrar_por_edad_ciudad() {
        let personas= crear_vector_con_personas();

        let persona= Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};
        let filtrados = persona.filtrar_por_edad_ciudad(&personas, 20, "La plata".to_string());

        //In filtrados there should be 2 people.
        assert!(filtrados.is_some());
        assert_eq!(filtrados.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_todos_viven_en() {
        let personas_no_cumplen = crear_vector_con_personas(); //Not everyone lives in La Plata.
        let personas_cumplen = vec![
            Persona {
                nombre: "Juli",
                apellido: "M",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 19,
                salario: 10000.0,
            },
            Persona {
                nombre: "Pepe",
                apellido: "P",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 20,
                salario: 10000.0,
            },
            Persona {
                nombre: "Nombre",
                apellido: "N",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 60,
                salario: 50000.0,
            },
            Persona {
                nombre: "Jorgito",
                apellido: "J",
                direccion: "Arg",
                ciudad: "La Plata",
                edad: 35,
                salario: 45000.0,
            },
            Persona {
                nombre: "Benja",
                apellido: "M",
                direccion: "Arg",
                ciudad: "La PLATA",
                edad: 25,
                salario: 60000.0,
            },
        ];

        let persona= Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};

        assert_eq!(persona.todos_viven_en(&personas_no_cumplen, "La plata"), false);
        assert_eq!(persona.todos_viven_en(&personas_cumplen, "La plata"), true);

    }

    #[test]
    fn test_alguno_vive_en() {
        let personas_cumplen = crear_vector_con_personas(); //There's one person that lives in Bs As (the city I'm using to check).
        let personas_no_cumplen = vec![ //Nobody lives in Bs As.s
            Persona {
                nombre: "Juli",
                apellido: "M",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 19,
                salario: 10000.0,
            },
            Persona {
                nombre: "Pepe",
                apellido: "P",
                direccion: "Arg",
                ciudad: "La plata",
                edad: 20,
                salario: 10000.0,
            },
        ];

        let persona= Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};

        assert_eq!(persona.alguno_vive_en(&personas_cumplen, "bs as"), true);
        assert_eq!(persona.alguno_vive_en(&personas_no_cumplen, "bs as"), false);

        assert_ne!(persona.alguno_vive_en(&personas_no_cumplen, "bs as"), true);

    }

    #[test]
    fn test_persona_existe() {
        let personas = crear_vector_con_personas();
        let persona_existente = Persona {nombre: "Juli", apellido: "M", ciudad: "la plata", direccion: "arg", edad: 19, salario: 10000.0};
        let persona_inexistente = Persona {nombre: "zzz", apellido: "M", ciudad: "la plata", direccion: "arg", edad: 99, salario: 20000.0};

        let persona_tester = Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};

        assert_eq!(persona_tester.persona_existe(&personas, &persona_existente), true);
        assert_eq!(persona_tester.persona_existe(&personas, &persona_inexistente), false);
    }

    #[test]
    fn test_recolectar_edades() {
        let personas = crear_vector_con_personas();
        let persona_tester = Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};

    
        let edades =  persona_tester.recolectar_edades(&personas);
        assert!(edades.is_some());
        assert_eq!(*edades.as_ref().unwrap(), vec![19,20,60,35,25]);


        let edades = persona_tester.recolectar_edades(&vec![]);
        assert!(edades.is_none());
    }

    #[test]
    fn test_salario_mayor_menor() {
        let personas = crear_vector_con_personas();
        let persona_tester = Persona {nombre: "Persona test", salario: 0.0, apellido: "test", edad: 0, ciudad: "", direccion: ""};

        let result = persona_tester.salario_mayor_menor(&personas);
        assert!(result.is_some());

        let (persona_menor_salario, persona_mayor_salario) = result.unwrap();

        assert_eq!(persona_menor_salario.nombre, "Pepe");
        assert_eq!(persona_mayor_salario.nombre, "Benja");


        let personas2: Vec<Persona<'_>> = vec![];
        let result2 = persona_tester.salario_mayor_menor(&personas2);
        assert!(result2.is_none());
    }
}