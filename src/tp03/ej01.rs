struct Persona {
    age: u32,
    name: String,
    direction: Option<String>,
}

impl Persona {
    fn new (age: u32, name: String, direction: Option<String>) -> Persona {
        Persona {
            age,
            name,
            direction,
        }
    }

    fn to_string (&self) -> String {
        let direction_str = if let Some(dir) = &self.direction{
            dir.as_str()
        } else {
            "None"
        };

        format! ("Name: {}, age: {}, direction: {}", self.name, self.age, direction_str)
    }

    fn obtener_edad (&self) -> u32 {
        self.age
    }

    fn actualizar_direccion (&mut self, new_direction: Option<String>){
        self.direction = new_direction;
    }

}

//# [should_panic]
# [test]
fn tester(){
    let mut me_without_direction = Persona::new (19, String::from("Julieta"), None);
    let mut me_with_direction = Persona::new (19, String::from("Julieta"), Some (String::from("Salto")));

    let message1 = me_with_direction.to_string();
    println!("{}", message1); //How do I do whis w/o the println! macro???
    println!("{}", me_without_direction.to_string());

    assert_eq!(me_with_direction.obtener_edad(), 19);

    me_with_direction.actualizar_direccion(Some(String::from("La Plata/Salto")));
    println!("{}", me_with_direction.to_string());
}