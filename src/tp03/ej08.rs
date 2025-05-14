#[derive(Clone)]
enum Genero {
    ROCK,
    POP,
    JAZZ,
    RAP,
    OTROS,
}

#[derive(Clone)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

struct PlayList {
    nombre: String,
    canciones: Vec<Cancion>,
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }
}

pub fn compare_genre(genero1: &Genero, genero2: &Genero) -> bool {
    match (genero1, genero2) {
        (Genero::JAZZ, Genero::JAZZ) => true,
        (Genero::ROCK, Genero::ROCK) => true,
        (Genero::RAP, Genero::RAP) => true,
        (Genero::POP, Genero::POP) => true,
        (Genero::OTROS, Genero::OTROS) => true,
        _ => false,
    }
}

pub fn compare (cancion1: &Cancion, cancion2: &Cancion) -> bool {
    cancion1.artista == cancion2.artista &&
    cancion1.titulo == cancion2.titulo &&
    compare_genre(&cancion1.genero, &cancion2.genero)

}

impl PlayList {
    fn new(nombre: String) -> PlayList {
        PlayList {
            nombre,
            canciones: Vec::new(),
        }
    }

    fn agregar_cancion(&mut self, cancion: &Cancion) {
        self.canciones.push(cancion.clone());
    }

    fn eliminar_cancion(&mut self, cancion: &Cancion) {
        //Find the song I want to delete in the vec.
        let mut index_song_delete= -1;
        for i in 0.. self.canciones.len() {
            if compare(&self.canciones[i], &cancion) {
                index_song_delete= i as i32;
                break;
            }  
        }
        //Delete the car.
        if index_song_delete != -1 { //If I found the car to delete.
            self.canciones.remove(index_song_delete as usize);
        }
    }

    //fn mover_cancion(&mut self, )

    fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<Cancion> {
        for song in self.canciones.iter() {
            if (song.titulo == nombre) {
                return Some(song.clone());
            }
        }
        None
    }

    fn obtener_canciones_por_genero(&self, genero: Genero) -> Vec<Cancion> {
        let mut canciones_genero= Vec::new();

        for song in self.canciones.iter() {
            if compare_genre(&song.genero, &genero) {
                canciones_genero.push(song.clone());
            }
        }
        canciones_genero
    }

    fn obtener_canciones_por_artista(&self, artista: String) -> Vec<Cancion> {
        let mut canciones_artista= Vec::new();

        for song in self.canciones.iter() {
            if song.artista == artista {
                canciones_artista.push(song.clone());
            }
        }

        canciones_artista
    }

    fn modificar_titulo_playlist(&mut self, titulo_nuevo: String) {
        self.nombre= titulo_nuevo;
    }

    fn eliminar_todas_las_canciones(&mut self) {
        self.canciones.clear();
    }
 }


 #[cfg(test)]

 mod test {
    use super::*;

    #[test]
    fn testear_agregar_cancion() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);
        let cancion2= Cancion::new(String::from("Cancion2"), String::from("juancito"), Genero::JAZZ);
        let cancion3= Cancion::new(String::from("Cancion3"), String::from("pepa"), Genero::OTROS);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        assert_eq!(playlist.canciones.len(), 3);
    }

    #[test]
    fn testear_eliminar_cancion() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);
        let cancion2= Cancion::new(String::from("Cancion2"), String::from("juancito"), Genero::JAZZ);
        let cancion3= Cancion::new(String::from("Cancion3"), String::from("pepa"), Genero::OTROS);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        
        playlist.eliminar_cancion(&cancion1); //Delete just 1 song.
        assert_eq!(playlist.canciones.len(), 1);

        //Try to delete a song that is not in the playlist.
        playlist.eliminar_cancion(&cancion3);
        assert_eq!(playlist.canciones.len(), 1); //lenght shouldn't change.
    }

    #[test]
    fn testear_buscar_cancion() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);

        playlist.agregar_cancion(&cancion1);

        //Search for a song that is in the playlist.
        assert!(playlist.buscar_cancion_por_nombre(String::from("Cancion1")).is_some());

        //Search for a song that is not in the playlist.
        assert!(playlist.buscar_cancion_por_nombre(String::from("Cancion10")).is_none());
    }

    #[test]
    fn testear_eliminar_canciones() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);
        let cancion2= Cancion::new(String::from("Cancion2"), String::from("juancito"), Genero::JAZZ);
        let cancion3= Cancion::new(String::from("Cancion3"), String::from("pepa"), Genero::OTROS);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        playlist.eliminar_todas_las_canciones();
        assert_eq!(playlist.canciones.len(), 0);
    }
 }