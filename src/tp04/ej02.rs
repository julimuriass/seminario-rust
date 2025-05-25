#[derive(Clone)]
struct Persona <'a>{
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

impl <'a> Persona <'a> {
    
    fn personas_con_mayor_salario<'b> (&self, personas: &'b Vec<Persona<'b>>, salario: f64) -> Option<Vec<Persona<'b>>> {
        let people:Vec<Persona> = personas.iter().filter(|p| p.salario > salario).cloned().collect();

        if people.len() == 0 {
            return None;
        } else {
            return Some(people);
        }
    }
}