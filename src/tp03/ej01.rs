struct Persona {
    edad: u32,
    nombre: String,
    direccion: Option<String>,
}

impl Persona {
    fn new (edad: u32, nombre: String, direccion: Option<String>) -> Persona {
        Persona {
            edad,
            nombre,
            direccion,
        }
    }

    fn to_string (&self) -> String {
        let direction_str = if let Some(dir) = &self.direccion{
            dir.as_str()
        } else {
            "None"
        };

        format! ("Name: {}, age: {}, direction: {}", self.nombre, self.edad, direction_str)
    }

    fn obtener_edad (&self) -> u32 {
        self.edad
    }

    fn actualizar_direccion (&mut self, new_direction: Option<String>){
        self.direccion = new_direction;
    }

}

//# [should_panic]
# [test]
fn tester(){
    let mut me_without_direction = Persona::new (19, String::from("Julieta"), None);
    let mut me_with_direction = Persona::new (19, String::from("Julieta"), Some (String::from("Salto")));

    let message1 = me_with_direction.to_string();
    //println!("{}", message1); 
    println!("{}", me_without_direction.to_string());

    assert_eq!(me_with_direction.obtener_edad(), 19);

    me_with_direction.actualizar_direccion(Some(String::from("La Plata/Salto")));
    println!("{}", me_with_direction.to_string());
}