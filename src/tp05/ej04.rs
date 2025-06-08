use std::collections::HashMap;
use crate::tp03::ej03::Fecha;

use serde::{Serialize, Deserialize};
use core::arch;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}

#[derive(Clone, Serialize, Deserialize)]
enum EstadoPrestamo {
    Devuelto,
    EnPrestamo,
}

#[derive(Clone)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: HashMap<u32, Libro>, //Key: ISBN.
    prestamos: Vec<Prestamo>,
    archivo_libros: PathBuf,
    archivo_prestamos: PathBuf,
}

#[derive(Clone, Serialize, Deserialize)]
struct Libro {
    isbn: u32,
    titulo: String,
    copias_disponiles: u32,
    autor: String,
    numero_paginas: u32,
    genero: Genero,
}

#[derive(Clone, Serialize, Deserialize)]
struct Prestamo {
    isbn_libro: u32,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Fecha, 
    estado: EstadoPrestamo,
}

#[derive(Clone, Serialize, Deserialize)]
struct Cliente {
    nombre: String,
    telefono: u32,
    correo: String,
}

pub fn no_devolvio (estado: &EstadoPrestamo) -> bool {
    match estado {
        (EstadoPrestamo::EnPrestamo) => true,
        _ => false,
    }
}

pub fn compare_clientes (cliente1: &Cliente, cliente2: &Cliente) -> bool {
    cliente1.nombre == cliente2.nombre &&
    cliente1.correo == cliente2.correo &&
    cliente1.telefono == cliente2.telefono
}

enum ErroresPersonalizados {
    ErrorArchivo,
}

impl Biblioteca {

    pub fn cargar_al_archivo_libros(&mut self) -> Result<(), ErroresPersonalizados> {
        let mut archivo_libros:File = match File::create(self.archivo_libros.clone()) {
            Err(e) => Err(ErroresPersonalizados::ErrorArchivo)?,
            Ok(arch) => arch,
        };

        serde_json::to_writer(&archivo_libros, &self.libros)
            .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

        Ok(())
    }

    pub fn cargar_al_archivo_prestamos(&mut self) -> Result<(), ErroresPersonalizados> {
        let mut archivo_prestamos:File = match File::create(self.archivo_prestamos.clone()) {
            Err(e) => Err(ErroresPersonalizados::ErrorArchivo)?,
            Ok(arch) => arch,
        };

        serde_json::to_writer(&archivo_prestamos, &self.prestamos)
            .map_err(|e| ErroresPersonalizados::ErrorArchivo)?;

        Ok(())
    }

    fn obtener_cantidad_copias (&self, libro: &Libro) -> u32 {
        if let Some(book) = self.libros.get(&libro.isbn) {
            return book.copias_disponiles;
        } else {
            return 0;
        } 
    }

    fn decrementar_cantidad_copias (&mut self, libro: &Libro) {
        if let Some(book) = self.libros.get_mut(&libro.isbn) {
            book.copias_disponiles -= 1;
        } 
        //Mod arch libro
    }

    fn incrementar_cantidad_copias (&mut self, libro: &Libro) {
        if let Some(book) = self.libros.get_mut(&libro.isbn) {
            book.copias_disponiles += 1;            
        }
        //Mod arch libro
    }

    fn contar_prestamos_cliente (&self, cliente: &Cliente) -> u32 {
        let mut cant_prestamos= 0;

        for prestamo in self.prestamos.iter() {
            if compare_clientes(&prestamo.cliente, &cliente) {
                cant_prestamos += 1;
            }
        }
        cant_prestamos
    }

    fn realizar_prestamo (&mut self, libro: &Libro, cliente: &Cliente) -> bool {
        //Check if the book exists and has available copies.
        let okay_prestamo = if let Some(book) = self.libros.get(&libro.isbn) {
            book.copias_disponiles >= 1
        } else {
            false
        };

        //Check if the client has fewer than 5 loans.
        if okay_prestamo && self.contar_prestamos_cliente(cliente) < 5 {
            if let Some(book) = self.libros.get_mut(&libro.isbn) {
                book.copias_disponiles -= 1;
                return true;
            }
        }
        false

        //Mod arch prest
    }


    fn prestamos_vencer(&self, cant_dias: u32, fecha_actual: &Fecha) -> Vec<Prestamo> {
        let mut fecha_limite = fecha_actual.clone();
        fecha_limite.sumar_dias(cant_dias);

        let mut lista_prestamos_vencer = Vec::new();

        for prestamo in self.prestamos.iter() {
            let fecha_igual = prestamo.fecha_vencimiento.dia == fecha_actual.dia &&
                          prestamo.fecha_vencimiento.mes == fecha_actual.mes &&
                          prestamo.fecha_vencimiento.año == fecha_actual.año;

            if no_devolvio(&prestamo.estado) &&
            !prestamo.fecha_vencimiento.es_mayor(&fecha_limite) &&  // fecha_vencimiento <= fecha_limite
            (prestamo.fecha_vencimiento.es_mayor(&fecha_actual) || fecha_igual)  // fecha_vencimiento >= fecha_actual
            {
                lista_prestamos_vencer.push(prestamo.clone());
            }
        }
         lista_prestamos_vencer    
    }

    fn prestamos_vencidos(&self, fecha_actual: &Fecha) -> Vec<Prestamo> {
        let mut lista_prestamos_vencidos = Vec::new();

        for prestamo in self.prestamos.iter() {
            if no_devolvio(&prestamo.estado) &&
            !prestamo.fecha_vencimiento.es_mayor(&fecha_actual)
            {
                lista_prestamos_vencidos.push(prestamo.clone());
            }
        }
        lista_prestamos_vencidos 
    }

    fn buscar_prestamo (&self, libro: &Libro, cliente: &Cliente) -> Option<Prestamo> {
        for prestamo in self.prestamos.iter() {
            if compare_clientes(&prestamo.cliente, &cliente) && prestamo.isbn_libro == libro.isbn {
                return Some(prestamo.clone());
            }
        }
        None
    }

    fn devolver_libro (&mut self, libro: &Libro, cliente: &Cliente) {
        for prestamo in self.prestamos.iter_mut() {
            if compare_clientes(&prestamo.cliente, &cliente) && prestamo.isbn_libro == libro.isbn {
                prestamo.estado= EstadoPrestamo::Devuelto;
                if let Some(book) = self.libros.get_mut(&libro.isbn) {
                    book.copias_disponiles += 1;                    
                }
                break;
            }
        }

        //Mod arch libro (copias)
        //Mod arch prest.
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obtener_cant_copias_libro() {
        let libro = Libro {
            isbn: 100,
            titulo: "Test Libro".to_string(),
            copias_disponiles: 7,
            autor: "Autor X".to_string(),
            numero_paginas: 200,
            genero: Genero::Novela,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();
    

        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Arg".to_string(),
            libros: HashMap::new(),
            prestamos: vec![],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };
    
        //Insertar libro.
        biblioteca.libros.insert(libro.isbn, libro.clone());
        let copias = biblioteca.obtener_cantidad_copias(&libro);
        assert_eq!(copias, 7);
    
        //Libro no existente debe devolver 0
        let libro_inexistente = Libro {
            isbn: 999,
            titulo: "No existe".to_string(),
            copias_disponiles: 0,
            autor: "".to_string(),
            numero_paginas: 0,
            genero: Genero::Otros,
        };
        let copias_inexistente = biblioteca.obtener_cantidad_copias(&libro_inexistente);
        assert_eq!(copias_inexistente, 0);
    }

    #[test]
    fn test_decrementar_copias() {
        let libro = Libro {
            isbn: 100,
            titulo: "Test Libro".to_string(),
            copias_disponiles: 7,
            autor: "Autor X".to_string(),
            numero_paginas: 200,
            genero: Genero::Novela,
        };
    

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();

        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Arg".to_string(),
            libros: HashMap::new(),
            prestamos: vec![],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };

        biblioteca.libros.insert(libro.isbn, libro.clone());
        biblioteca.decrementar_cantidad_copias(&libro);
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 6);
        
    }

    #[test]
    fn test_incrementar_copias() {
        let libro = Libro {
            isbn: 100,
            titulo: "Test Libro".to_string(),
            copias_disponiles: 7,
            autor: "Autor X".to_string(),
            numero_paginas: 200,
            genero: Genero::Novela,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();
    
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Arg".to_string(),
            libros: HashMap::new(),
            prestamos: vec![],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };

        biblioteca.libros.insert(libro.isbn, libro.clone());
        biblioteca.incrementar_cantidad_copias(&libro);
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 8);
        
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let cliente = Cliente {
            nombre: "Ana".to_string(),
            telefono: 123456,
            correo: "ana@mail.com".to_string(),
        };
    
        let libro1 = Libro {
            isbn: 1,
            titulo: "Libro 1".to_string(),
            copias_disponiles: 3,
            autor: "Autor 1".to_string(),
            numero_paginas: 100,
            genero: Genero::Novela,
        };
    
        let libro2 = Libro {
            isbn: 2,
            titulo: "Libro 2".to_string(),
            copias_disponiles: 2,
            autor: "Autor 2".to_string(),
            numero_paginas: 200,
            genero: Genero::Infantil,
        };
    
        let prestamo1 = Prestamo {
            isbn_libro: libro1.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 1, mes: 1, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };
    
        let prestamo2 = Prestamo {
            isbn_libro: libro2.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 10, mes: 1, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::Devuelto,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();
    
        let biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Calle Falsa".to_string(),
            libros: HashMap::new(),
            prestamos: vec![prestamo1, prestamo2],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };
    
        let cantidad = biblioteca.contar_prestamos_cliente(&cliente);
        assert_eq!(cantidad, 2);
    }

    #[test]
    fn test_realizar_prestamo() {
        let cliente = Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            correo: "juan@mail.com".to_string(),
        };
    
        let libro = Libro {
            isbn: 101,
            titulo: "Libro Test".to_string(),
            copias_disponiles: 3,
            autor: "Autor Test".to_string(),
            numero_paginas: 150,
            genero: Genero::Novela,
        };
    

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();

        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Calle Test".to_string(),
            libros: HashMap::new(),
            prestamos: vec![],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };
    
        biblioteca.libros.insert(libro.isbn, libro.clone());
    
        // Cliente sin préstamos previos, debería poder tomar prestado
        let exito = biblioteca.realizar_prestamo(&libro, &cliente);
        assert!(exito);
    
        // Copias deberían reducirse en 1
        let copias_restantes = biblioteca.obtener_cantidad_copias(&libro);
        assert_eq!(copias_restantes, libro.copias_disponiles - 1);
    
        // Simulamos que el cliente ya tiene 5 préstamos
        for _ in 0..5 {
            biblioteca.prestamos.push(Prestamo {
                isbn_libro: 999, // libro cualquiera
                cliente: cliente.clone(),
                fecha_vencimiento: Fecha { dia: 1, mes: 1, año: 2025 },
                fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
                estado: EstadoPrestamo::EnPrestamo,
            });
        }
    
        // Ahora no debería permitir más préstamos
        let no_exito = biblioteca.realizar_prestamo(&libro, &cliente);
        assert!(!no_exito);
    
        // Copias no deberían cambiar porque no se hizo el préstamo
        let copias_final = biblioteca.obtener_cantidad_copias(&libro);
        assert_eq!(copias_final, libro.copias_disponiles - 1);
    }

    #[test]
    fn test_prestamos_a_vencer () {
        let fecha_actual = Fecha { dia: 10, mes: 5, año: 2025 };

        let cliente = Cliente {
            nombre: "Luis".to_string(),
            telefono: 5551234,
            correo: "luis@mail.com".to_string(),
        };

        let libro = Libro {
            isbn: 123,
            titulo: "Libro de prueba".to_string(),
            copias_disponiles: 2,
            autor: "Autor X".to_string(),
            numero_paginas: 150,
            genero: Genero::Novela,
        };

         // Préstamo que vence dentro de 5 días (día 14)
        let prestamo_proximo = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 14, mes: 5, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };

        // Préstamo que vence después de 10 días (día 22)
        let prestamo_lejano = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 22, mes: 5, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };

        // Préstamo ya devuelto, no debería aparecer
        let prestamo_devuelto = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 12, mes: 5, año: 2025 },
            fecha_devolucion: Fecha { dia: 11, mes: 5, año: 2025 },
            estado: EstadoPrestamo::Devuelto,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();

        let biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "arg".to_string(),
            libros: HashMap::new(),
            prestamos: vec![prestamo_proximo.clone(), prestamo_lejano, prestamo_devuelto],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };

        let prestamos_vencer = biblioteca.prestamos_vencer(5, &fecha_actual);

        // Sólo debería contener el préstamo que vence dentro de 5 días y está activo
        assert_eq!(prestamos_vencer.len(), 1);

    }

    #[test]
    fn test_prestamos_vencidos() {
        let fecha_actual = Fecha { dia: 15, mes: 5, año: 2025 };

        let cliente = Cliente {
          nombre: "Carlos".to_string(),
          telefono: 123456,
          correo: "carlos@mail.com".to_string(),
        };

        let libro = Libro {
            isbn: 1,
            titulo: "Libro X".to_string(),
            copias_disponiles: 1,
            autor: "Autor X".to_string(),
            numero_paginas: 100,
            genero: Genero::Novela,
        };

        let prestamo_vencido = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 10, mes: 5, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };

        let prestamo_no_vencido = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 20, mes: 5, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };

        let prestamo_devuelto = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 5, mes: 5, año: 2025 },
            fecha_devolucion: Fecha { dia: 6, mes: 5, año: 2025 },
            estado: EstadoPrestamo::Devuelto,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();

        let biblioteca = Biblioteca {
            nombre: "Biblio Test".to_string(),
            direccion: "Dirección".to_string(),
            libros: HashMap::new(),
            prestamos: vec![
                prestamo_vencido.clone(),
                prestamo_no_vencido,
                prestamo_devuelto,
            ],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };

        let vencidos = biblioteca.prestamos_vencidos(&fecha_actual);

        assert_eq!(vencidos.len(), 1);
    }

    #[test]
    fn test_buscar_prestamo() {
        let cliente = Cliente {
            nombre: "Ana".to_string(),
            telefono: 123456,
            correo: "ana@mail.com".to_string(),
        };
    
        let libro1 = Libro {
            isbn: 1,
            titulo: "Libro 1".to_string(),
            copias_disponiles: 3,
            autor: "Autor 1".to_string(),
            numero_paginas: 100,
            genero: Genero::Novela,
        };
    
        let libro2 = Libro {
            isbn: 2,
            titulo: "Libro 2".to_string(),
            copias_disponiles: 2,
            autor: "Autor 2".to_string(),
            numero_paginas: 200,
            genero: Genero::Infantil,
        };
    
        let prestamo1 = Prestamo {
            isbn_libro: libro1.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 1, mes: 1, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();
    

        let biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Calle Falsa".to_string(),
            libros: HashMap::new(),
            prestamos: vec![prestamo1],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };
    
        assert!(biblioteca.buscar_prestamo(&libro1, &cliente).is_some());

        //Busco uno que no exista;
        assert!(biblioteca.buscar_prestamo(&libro2, &cliente).is_none())
    }

    #[test]
    fn test_devolver_libro() {
        let cliente = Cliente {
            nombre: "Roberto".to_string(),
            telefono: 555123789,
            correo: "roberto@mail.com".to_string(),
        };
    
        
        let libro = Libro {
            isbn: 42,
            titulo: "pepe".to_string(),
            copias_disponiles: 5,  // Inicialmente hay 5 copias
            autor: "Autor".to_string(),
            numero_paginas: 300,
            genero: Genero::Novela,
        };
    
        
        let prestamo = Prestamo {
            isbn_libro: libro.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 15, mes: 6, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },  // Aún no devuelto
            estado: EstadoPrestamo::EnPrestamo,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();
    
        
        let mut biblioteca = Biblioteca {
            nombre: "biblioteca".to_string(),
            direccion: "arg".to_string(),
            libros: HashMap::new(),
            prestamos: vec![prestamo],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };

        biblioteca.devolver_libro(&libro, &cliente);//Lo devuelvo.
        let libro_devuelto = biblioteca.buscar_prestamo(&libro, &cliente);
        let esta_devuelto = {
            match libro_devuelto.unwrap().estado {
                (EstadoPrestamo::Devuelto) => true,
                _ => false,
                
            }
        };
        assert_eq!(esta_devuelto, true);
    }

    //Tests nuevos.
    #[test]
    fn test_cargar_al_archivo_libros() {
        let libro1 = Libro {
            isbn: 1,
            titulo: "Libro 1".to_string(),
            copias_disponiles: 3,
            autor: "Autor 1".to_string(),
            numero_paginas: 100,
            genero: Genero::Novela,
        };
    
        let libro2 = Libro {
            isbn: 2,
            titulo: "Libro 2".to_string(),
            copias_disponiles: 2,
            autor: "Autor 2".to_string(),
            numero_paginas: 200,
            genero: Genero::Infantil,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();

        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Calle Falsa".to_string(),
            libros: HashMap::new(),
            prestamos: vec![],
            archivo_libros: path_books.clone().into(),
            archivo_prestamos: path_prestamos.clone().into(),
        };

        biblioteca.libros.insert(1, libro1.clone());
        biblioteca.libros.insert(2, libro2.clone());

        assert_eq!(biblioteca.libros.len(), 2);
        assert!(biblioteca.cargar_al_archivo_libros().is_ok()); //Ok.
    }

    #[test]
    fn test_cargar_al_archivo_prestamos() {
        let cliente = Cliente {
            nombre: "Ana".to_string(),
            telefono: 123456,
            correo: "ana@mail.com".to_string(),
        };
    
        let libro1 = Libro {
            isbn: 1,
            titulo: "Libro 1".to_string(),
            copias_disponiles: 3,
            autor: "Autor 1".to_string(),
            numero_paginas: 100,
            genero: Genero::Novela,
        };
    
        let prestamo1 = Prestamo {
            isbn_libro: libro1.isbn,
            cliente: cliente.clone(),
            fecha_vencimiento: Fecha { dia: 1, mes: 1, año: 2025 },
            fecha_devolucion: Fecha { dia: 0, mes: 0, año: 0 },
            estado: EstadoPrestamo::EnPrestamo,
        };

        let path_books = "src/tp05/archivo_libros.txt".to_string();
        let path_prestamos = "src/tp05/archivo_prestamos.txt".to_string();
    

        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca Test".to_string(),
            direccion: "Calle Falsa".to_string(),
            libros: HashMap::new(),
            prestamos: vec![prestamo1],
            archivo_libros: path_books.into(),
            archivo_prestamos: path_prestamos.into(),
        };

        assert!(biblioteca.cargar_al_archivo_prestamos().is_ok());
    }
}