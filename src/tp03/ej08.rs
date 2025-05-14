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

    fn mover_cancion(&mut self, cancion: &Cancion, posicion_nueva: u32){
        if posicion_nueva < self.canciones.len() as u32 {

            //Search song in the playlist.
            let mut song_index:i32= -1;
            for i in 0..self.canciones.len() {
                if compare(&self.canciones[i], &cancion) {
                    song_index= i as i32;
                    break;
                }
            }

            if song_index != -1 { //If I found the song.
                //Move the song to the new position.
                let song = self.canciones.remove(song_index as usize);
                self.canciones.insert(posicion_nueva as usize, song);
            }
        }
    }

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

    #[test]
    fn testear_mover_cancion() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);
        let cancion2= Cancion::new(String::from("Cancion2"), String::from("juancito"), Genero::JAZZ);
        let cancion3= Cancion::new(String::from("Cancion3"), String::from("pepa"), Genero::OTROS);
        let cancion4= Cancion::new(String::from("Cancion4"), String::from("pepa"), Genero::ROCK);

        playlist.agregar_cancion(&cancion1); //Originally in pos 0 (in playlist).
        playlist.agregar_cancion(&cancion2); //Originally in pos 1 (in playlist).
        playlist.agregar_cancion(&cancion3); //Originally in pos 2 (in playlist).

        //Check that.
        assert_eq!(playlist.canciones[0].titulo, String::from("Cancion1"));

        //Move a song that is in the playlist.
        playlist.mover_cancion(&cancion2, 2);
        assert_eq!(playlist.canciones[2].titulo, String::from("Cancion2"));

        //Move a song that is not in the playlist.
        playlist.mover_cancion(&cancion4, 0);
        assert_eq!(playlist.canciones[0].titulo, String::from("Cancion1")); //It shouldn't change.

        //Move a song that is in the playlist to a non existing position.
        playlist.mover_cancion(&cancion1, 3);
        assert_eq!(playlist.canciones[0].titulo, String::from("Cancion1")); //It shouldn't change.
    }

    #[test]
    fn testear_cambiar_nombre_playlist() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        //Check that.
        assert_eq!(playlist.nombre, String::from("Chill music"));
        
        //Change the name.
        playlist.modificar_titulo_playlist(String::from("Nombre nuevo"));
        assert_eq!(playlist.nombre, String::from("Nombre nuevo"));
    }

    #[test]
    fn testear_canciones_mismo_genero() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);
        let cancion2= Cancion::new(String::from("Cancion2"), String::from("juancito"), Genero::JAZZ);
        let cancion3= Cancion::new(String::from("Cancion3"), String::from("pepa"), Genero::OTROS);
        let cancion4= Cancion::new(String::from("Cancion4"), String::from("pepa"), Genero::ROCK);

        playlist.agregar_cancion(&cancion1); 
        playlist.agregar_cancion(&cancion2); 
        playlist.agregar_cancion(&cancion3); 
        playlist.agregar_cancion(&cancion4);

        //2 jazz songs.
        let mut jazz_songs: Vec<Cancion>;
        jazz_songs= playlist.obtener_canciones_por_genero(Genero::JAZZ);
        assert_eq!(jazz_songs.len(), 2);

        //Try with a genre that is not in the playlist.
        let mut pop_songs= playlist.obtener_canciones_por_genero(Genero::POP);
        assert_eq!(pop_songs.len(), 0);
    }

    #[test]
    fn testear_canciones_mismo_artista() {
        let mut playlist= PlayList::new(String::from("Chill music"));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);
        let cancion2= Cancion::new(String::from("Cancion2"), String::from("juancito"), Genero::JAZZ);
        let cancion3= Cancion::new(String::from("Cancion3"), String::from("pepa"), Genero::OTROS);
        let cancion4= Cancion::new(String::from("Cancion4"), String::from("pepa"), Genero::ROCK);

        playlist.agregar_cancion(&cancion1); 
        playlist.agregar_cancion(&cancion2); 
        playlist.agregar_cancion(&cancion3); 
        playlist.agregar_cancion(&cancion4);

        //2 pepa songs.
        let mut pepa_songs: Vec<Cancion>;
        pepa_songs= playlist.obtener_canciones_por_artista(String::from("pepa"));
        assert_eq!(pepa_songs.len(), 2);

        //Try with an artist that is not in the playlist.
        let mut titi_songs= playlist.obtener_canciones_por_artista(String::from("titi"));
        assert_eq!(titi_songs.len(), 0);
    }
 }