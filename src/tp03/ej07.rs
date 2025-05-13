use std::collections::LinkedList;


enum Color {
    ROJO,
    VERDE, 
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}
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
    autos: LinkedList<Auto>,
}

/*
➢​ eliminar_auto(auto): elimina un auto de la lista de autos.

➢​ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖​ Auto:
➢​ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna.
➢​ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■​ si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%.
■​ si la marca es BMW le aplica un recargo del 15%-
■​ si el año es menor a 2000 le aplica un descuento del 5%. */

impl ConsecionarioAuto {
    fn new(nombre: String, direccion: String, x: u32) -> ConsecionarioAuto {
        ConsecionarioAuto {
            nombre,
            direccion,
            x,
            autos: LinkedList::new(),
        }
    }

    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len()+1 <= self.x.try_into().unwrap() {
            self.autos.push_back(auto);
            return true;
        } else {
            return false;
        }
    }

    fn eliminar_auto(&mut self, auto: Auto) {
        self.autos.retain(|a| *a != auto);
    }


}