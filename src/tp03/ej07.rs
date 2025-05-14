#[derive(Clone)]

enum Color {
    ROJO,
    VERDE, 
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}
#[derive(Clone)]
struct Auto {
    color: Color,
    marca: String,
    modelo: String,
    precio_bruto: f64,
    año: u32,
}

struct ConsecionarioAuto {
    nombre: String,
    direccion: String,
    x: u32,
    autos: Vec<Auto>,
}



pub fn compare_colors(color1: &Color, color2: &Color) -> bool {
    match(color1, color2) {
        (Color::AMARILLO, Color::AMARILLO) => true,
        (Color::AZUL, Color::AZUL) => true,
        (Color::VERDE, Color::VERDE) => true,
        (Color::ROJO, Color::ROJO) => true,
        (Color::NEGRO, Color::NEGRO) => true,
        (Color::BLANCO, Color::BLANCO) => true,
        _ => false, //If the variants are different, they are not equal
    }
}

pub fn compare (auto1: &Auto, auto2: &Auto) -> bool {
    auto1.año == auto2.año &&
    auto1.marca == auto2.marca &&
    auto1.modelo == auto2.modelo &&
    auto1.precio_bruto == auto2.precio_bruto &&
    compare_colors(&auto1.color, &auto2.color)

}

impl ConsecionarioAuto {
    fn new(nombre: String, direccion: String, x: u32) -> ConsecionarioAuto {
        ConsecionarioAuto {
            nombre,
            direccion,
            x,
            autos: Vec::new(),
        }
    }

    fn agregar_auto(&mut self, auto: &Auto) -> bool {
        if self.autos.len()+1 <= self.x.try_into().unwrap() {
            self.autos.push(auto.clone());
            return true;
        } else {
            return false;
        }
    }

    fn eliminar_auto(&mut self, auto: &Auto) {
        //Find the car I want to delete in the vec.
        let mut index_car_delete= -1;
        for i in 0.. self.autos.len() {
            if compare(&self.autos[i], &auto) {
                index_car_delete= i as i32;
                break;
            }  
        }
        //Delete the car.
        if index_car_delete != -1 { //If I found the car to delete.
            self.autos.remove(index_car_delete as usize);
        }
    }

    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for car in self.autos.iter() {
            if compare(&car, &auto) {
                return Some(car);
            }
        }
        None //If I didn't find it, return nothing (None).
    }
}

impl Auto {
    fn new(marca: String, modelo: String, precio_bruto: f64, año: u32, color: Color ) -> Auto {
        Auto {
            marca,
            modelo,
            precio_bruto,
            año,
            color,
        }
    }

    fn calcular_precio(&self) -> f64 {
        let mut precio_final= self.precio_bruto;

        //Check first condition (color).
        let es_primario = match self.color {
            Color::AMARILLO | Color::AZUL | Color::ROJO => true,
            _ => false,
        };

        if es_primario {
            precio_final += (self.precio_bruto * 25.0)/100.0; //+25%.
        } else {
            precio_final -= (10.0*self.precio_bruto)/100.0; //-10%.
        }

        //Check second condition (brand).
        if self.marca == "BMW" {
            precio_final += (self.precio_bruto * 15.0)/100.0; //+15%.
        }

        //Check third condition (year).
        if self.año < 2000 {
            precio_final -= (self.precio_bruto * 5.0)/100.0; //-5%.
        }

        precio_final
    }
}

#[cfg(test)]
mod  test {
    use super::*;

    #[test]
    fn testear_agregar_auto() {
        let mut concesionario= ConsecionarioAuto::new(String::from("Juli cars :D"), String::from("Argentina"), 3);

        let auto1= Auto::new(String::from("BMW"), String::from("modelo J"), 1000.0, 2015, Color::AMARILLO);
        let auto2= Auto::new(String::from("Honda"), String::from("modelo H"), 1000.0, 2015, Color::NEGRO);
        let auto3= Auto::new(String::from("Cronos"), String::from("modelo X"), 1000.0, 2015, Color::ROJO);
        let auto4= Auto::new(String::from("Ferrari"), String::from("modelo R"), 1000.0, 2015, Color::ROJO);

        concesionario.agregar_auto(&auto1);
        assert_eq!(concesionario.autos.len(), 1); //Added only 1 element.

        //Fill all available space.
        concesionario.agregar_auto(&auto2);
        concesionario.agregar_auto(&auto3);
        assert_eq!(concesionario.autos.len(), 3);

        //Try to add one more.
        assert_eq!(concesionario.agregar_auto(&auto4), false);
    }

    #[test]
    fn testear_eliminar_auto() {
        let mut concesionario= ConsecionarioAuto::new(String::from("Juli cars :D"), String::from("Argentina"), 3);
        let auto1= Auto::new(String::from("BMW"), String::from("modelo J"), 1000.0, 2015, Color::AMARILLO);
        let auto2= Auto::new(String::from("Honda"), String::from("modelo H"), 1000.0, 2015, Color::NEGRO);

        //First I try to delete a car from an empty list.
        concesionario.eliminar_auto(&auto1);
        assert_eq!(concesionario.autos.len(), 0); //Passed. Nothing changed.

        //Add cars to delete.
        concesionario.agregar_auto(&auto1);
        concesionario.eliminar_auto(&auto1);
        assert_eq!(concesionario.autos.len(), 0); //yes.

        concesionario.agregar_auto(&auto1);
        concesionario.agregar_auto(&auto2);
        concesionario.eliminar_auto(&auto1);
        assert_eq!(concesionario.autos.len(), 1);
    }
    
    #[test]
    fn testear_buscar_auto() {
        let mut concesionario= ConsecionarioAuto::new(String::from("Juli cars :D"), String::from("Argentina"), 3);
        let auto1= Auto::new(String::from("BMW"), String::from("modelo J"), 1000.0, 2015, Color::AMARILLO);
        let auto2= Auto::new(String::from("Honda"), String::from("modelo H"), 1000.0, 2015, Color::NEGRO);
        let auto3= Auto::new(String::from("Cronos"), String::from("modelo X"), 1000.0, 2015, Color::ROJO);


        //Try to find a car in an empty list.
        assert!(concesionario.buscar_auto(&auto1).is_none());

        //Find a car that exists.
        concesionario.agregar_auto(&auto1);
        concesionario.agregar_auto(&auto2);
        assert!(concesionario.buscar_auto(&auto2).is_some());

        //Find a car that doesn't exist.
        assert!(concesionario.buscar_auto(&auto3).is_none());
    }

    #[test]
    fn testear_calcular_precio() {
        //Test brand (and not primary color).
        let auto1= Auto::new(String::from("BMW"), String::from("modelo 1"), 1000.0, 2004, Color::BLANCO);
        let auto2= Auto::new(String::from("Toyota"), String::from("modelo 1"), 1000.0, 2004, Color::BLANCO);

        assert_eq!(auto1.calcular_precio(), 1050.0); 
        assert_eq!(auto2.calcular_precio(), 900.0);

        //Test if primary color.
        let auto3= Auto::new(String::from("Toyota"), String::from("modelo 1"), 1000.0, 2004, Color::AMARILLO);
        assert_eq!(auto3.calcular_precio(), 1250.0);
        
        //Test year (withouth primary color).
        let auto4= Auto::new(String::from("Cronos"), String::from("modelo 1"), 1000.0, 1999, Color::BLANCO);
        assert_eq!(auto4.calcular_precio(), 850.0);
    }

}