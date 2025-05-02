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

        formart! ("Name: {}, age: {}, direction: {}", self.name, self.age, direction_str)
    }

}