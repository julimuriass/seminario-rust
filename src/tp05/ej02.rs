use serde::{Serialize, Deserialize};
use core::arch;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;



#[derive(Clone, Serialize, Deserialize)]
enum Genero {
    ROCK,
    POP,
    JAZZ,
    RAP,
    OTROS,
}

#[derive(Clone, Serialize, Deserialize)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

struct PlayList {
    nombre: String,
    canciones: Vec<Cancion>,
    archivo_canciones: PathBuf,
}

enum ErroresPersonalizados {
    CancionNoEncontrada(String),
    ErrorArchivo(String),
    PosicionFueraDeRango,
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
    fn new(nombre: String, archivo_canciones: String) -> PlayList {
        let path = PathBuf::from(archivo_canciones);
        PlayList {
            nombre,
            canciones: Vec::new(),
            archivo_canciones: path,
        }
    }

    pub fn cargar_al_archivo(&mut self, cancion: &Cancion) -> Result<(), ErroresPersonalizados> {
        //Abrir el archivo en modo escritura.
        let mut archivo = match File::create(self.archivo_canciones.clone()) {
            Err(e) => Err(ErroresPersonalizados::ErrorArchivo(format!("Problema al abrir el archivo")))?,
            Ok(arch) => arch,            
        };

        let cancion_serializada = serde_json::to_string(&cancion).unwrap();
        match archivo.write(&cancion_serializada.as_bytes()) {
            Err(e) => Err(ErroresPersonalizados::ErrorArchivo(format!("Problema al escribir en el archivo")))?,
            Ok(_) => Ok(()),  
        }
    }

    fn agregar_cancion(&mut self, cancion: &Cancion) {
        self.canciones.push(cancion.clone());
        self.cargar_al_archivo(cancion); //Qué hago con el unused result? -> retornar un result.

    }

    fn eliminar_cancion(&mut self, cancion: &Cancion) -> Result<(), ErroresPersonalizados> {
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

            let file = std::fs::OpenOptions::new() //Me armo mi file nuevo.
                .read(true)
                .write(true)
                .open(&self.archivo_canciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo(format!("Error al abrir el archivo: {}", e)))?;

            self.canciones.remove(index_song_delete as usize);

            let writer = std::io::BufWriter::new(&file); //Crea un nuevo BufWriter.

            serde_json::to_writer(writer, &self.canciones)
                .map_err(|e| ErroresPersonalizados::ErrorArchivo(format!("Error al escribir en el archivo: {}", e)))?;
        
            Ok(())
        } else {
            Err(ErroresPersonalizados::CancionNoEncontrada(format!("NO se encontró la canción que desea eliminar.")))
        }
    }

    fn mover_cancion(&mut self, cancion: &Cancion, posicion_nueva: u32) -> Result<(), ErroresPersonalizados>{
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

                let file = std::fs::OpenOptions::new() //Me armo mi file nuevo.
                    .read(true)
                    .write(true)
                    .open(&self.archivo_canciones)
                    .map_err(|e| ErroresPersonalizados::ErrorArchivo(format!("Error al abrir el archivo: {}", e)))?;

                
                let writer = std::io::BufWriter::new(&file); //Crea un nuevo BufWriter.

                serde_json::to_writer(writer, &self.canciones)
                    .map_err(|e| ErroresPersonalizados::ErrorArchivo(format!("Error al escribir en el archivo: {}", e)))?;

                Ok(())
            } else {
                Err(ErroresPersonalizados::CancionNoEncontrada(format!("No se encontró la canción que desea mover.")))
            }

        } else {
            Err(ErroresPersonalizados::PosicionFueraDeRango)
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

    fn eliminar_todas_las_canciones(&mut self) -> Result<(), ErroresPersonalizados>{
        self.canciones.clear();

        let mut archivo = OpenOptions::new() 
            .write(true)
            .read(true)
            .open(self.archivo_canciones.clone())
            .map_err(|e| ErroresPersonalizados::ErrorArchivo(format!("Error al abrir el archivo: {}", e)))?;

        archivo.write_all(b"[]").map_err(|e| ErroresPersonalizados::ErrorArchivo(format!("Error al abrir el archivo: {}", e)))?; //b"[]" to write an empty JSON array to the file.

        Ok(())
    }
 }


 #[cfg(test)]

 mod test {
    use super::*;

    #[test]
    fn testear_agregar_cancion() {
        let path = "src/tp05/archivo_canciones.txt";
        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
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
        let path = "src/tp05/archivo_canciones.txt";

        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
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
        let path = "src/tp05/archivo_canciones.txt";
        
        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
        let cancion1= Cancion::new(String::from("Cancion1"), String::from("pepe"), Genero::JAZZ);

        playlist.agregar_cancion(&cancion1);

        //Search for a song that is in the playlist.
        assert!(playlist.buscar_cancion_por_nombre(String::from("Cancion1")).is_some());

        //Search for a song that is not in the playlist.
        assert!(playlist.buscar_cancion_por_nombre(String::from("Cancion10")).is_none());
    }

    #[test]
    fn testear_eliminar_canciones() { //No anda. help.
        let path = "src/tp05/archivo_canciones.txt";

        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
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
        let path = "src/tp05/archivo_canciones.txt";

        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
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
        let path = "src/tp05/archivo_canciones.txt";

        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
        //Check that.
        assert_eq!(playlist.nombre, String::from("Chill music"));
        
        //Change the name.
        playlist.modificar_titulo_playlist(String::from("Nombre nuevo"));
        assert_eq!(playlist.nombre, String::from("Nombre nuevo"));
    }

    #[test]
    fn testear_canciones_mismo_genero() {
        let path = "src/tp05/archivo_canciones.txt";

        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
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
        let path = "src/tp05/archivo_canciones.txt";

        let mut playlist= PlayList::new(String::from("Chill music"), String::from(path));
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