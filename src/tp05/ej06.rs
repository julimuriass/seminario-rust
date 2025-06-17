use std::{collections::HashMap, path::PathBuf};
use chrono::{DateTime, Utc};
use rand::Rng;
use std::ptr::eq;

use serde::{Serialize, Deserialize};
use core::arch;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: u32,
    identidad_validada: bool,
    balance_fiat: f64,
    balance_criptomoneda: HashMap<String, f64>, //Nombre_cripto -> cantidad.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Criptomoneda { 
    nombre: String,
    prefijo: String,
    listado_blockchains: Vec<Blockchain>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TipoTransaccion {
    IngresoFiat,
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    RetiroFiat,
}

impl PartialEq for TipoTransaccion {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TipoTransaccion::CompraCripto, TipoTransaccion::CompraCripto) => true,
            (TipoTransaccion::IngresoFiat, TipoTransaccion::IngresoFiat) => true,
            (TipoTransaccion::RecepcionCripto, TipoTransaccion::RecepcionCripto) => true,
            (TipoTransaccion::RetiroCripto, TipoTransaccion::RetiroCripto) => true,
            (TipoTransaccion::RetiroFiat, TipoTransaccion::RetiroFiat) => true,
            (TipoTransaccion::VentaCripto, TipoTransaccion::VentaCripto) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Medio {
    MercadoPago,
    TransferenciaBancaria,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Transaccion {
    fecha: DateTime<Utc>,
    tipo: TipoTransaccion,
    usuario: Usuario,
    monto_fiat: Option<f64>,
    criptomoneda: Option<Criptomoneda>,
    monto_criptomoneda: Option<f64>,
    cotizacion: Option<f64>,
    blockchain: Option<Blockchain>,
    hash: Option<String>,
    medio: Option<Medio>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlataformaXYZ {
    usuarios: HashMap<String, Usuario>, //Email -> usuario.
    criptomonedas: HashMap<String, Criptomoneda>, //Nombre_cripto -> cripto.
    transacciones: Vec<Transaccion>,
    archivo_transacciones: PathBuf,
    archivo_usuarios_balances: PathBuf,
}

//Custom errors.
#[derive(Clone, Debug)]
enum ErrorIntercambio { 
    UsuarioNoValido,
    BalanceInsuficiente,
    CriptoNoEncontrada,
    BlockchainNoDisponible,
    UsuarioNoEncontrado,
    ErrorArchivo, //está bien que esté dentro de errorIntercambio o tendría que estar en otro enum?
}

//Obtener cotizacion.
pub fn obtener_cotizacion(cripto_nombre: &str) -> f64  { //Datos en dólares del 28/05/2025.
    match cripto_nombre {
        "Bitcoin" => 107281.20,
        "Ethereum" => 18.10,
        "Litecoin" => 94.42,
        _ => 50.0,
    }
}

impl PlataformaXYZ {
    fn new(archivo_transacciones: String, archivo_usuarios_balances: String) -> Self {
        let path_trans = PathBuf::from(archivo_transacciones);
        let path_users_bal = PathBuf::from(archivo_usuarios_balances);
        PlataformaXYZ { usuarios: HashMap::new(), criptomonedas: HashMap::new(), transacciones:Vec::new(), archivo_transacciones: path_trans, archivo_usuarios_balances: path_users_bal }
    //Tendría que inicializar los archivos de transacciones y de usuarios_balances acá vacíos??? O así está bien?
    
    }

    fn cargar_transaccion_al_archivo(&mut self, transaccion: &Transaccion) -> Result<(), ErrorIntercambio> {
        let mut file = OpenOptions::new()
        .create(true)      // Create file if it doesn't exist
        .append(true)      // Append to end of file (don't overwrite)
        .open(&self.archivo_transacciones)
        .map_err(|_| ErrorIntercambio::ErrorArchivo)?;

        let transaccion_serializada = serde_json::to_string(&transaccion).unwrap();

        match file.write(&transaccion_serializada.as_bytes()) {
            Err(e) => Err(ErrorIntercambio::ErrorArchivo),
            Ok(_) => Ok(()),
        }
    }

    fn cargar_usuarios_al_archivo(&mut self) -> Result<(), ErrorIntercambio> {
        //Cargar todos los usuarios al archivo.
        let mut file:File = match File::create(self.archivo_usuarios_balances.clone()) {
            Err(e) => Err(ErrorIntercambio::ErrorArchivo)?,
            Ok(arch) => arch,
        };

        let writer = std::io::BufWriter::new(&file);

        serde_json::to_writer(writer, &self.usuarios) 
            .map_err(|e| ErrorIntercambio::ErrorArchivo)?;

        Ok(())
    }

    //Registrar usuario.
    pub fn registrar_usuario(&mut self, usuario: Usuario) { 
        self.usuarios.insert(usuario.email.clone(), usuario);
    }

    pub fn registrar_criptomoneda(&mut self, criptomoneda: Criptomoneda) {
        self.criptomonedas.insert(criptomoneda.nombre.clone(), criptomoneda);
    }

    pub fn ingresar_dinero(&mut self, monto_fiat: f64, usuario: &mut Usuario) -> Result<(), ErrorIntercambio> {
        //Verify that the user exists and it is valided.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        match usuario.identidad_validada {
            true => {
                //Acredito el monto fiat.
                usuario.balance_fiat += monto_fiat;
                //println!("{}", usuario.balance_fiat); Just to checked it worked (Ok.)
                //Creo la transaccion.
                let transaccion = Transaccion {
                    fecha: Utc::now(),
                    tipo: TipoTransaccion::IngresoFiat,
                    usuario: usuario.clone(),
                    monto_fiat: Some(monto_fiat),
                    criptomoneda: None,
                    monto_criptomoneda: None,
                    cotizacion: None,
                    blockchain: None,
                    hash: None,
                    medio: None,
                };

                //Agrego la transaccion a mi registro de transacciones.
                self.transacciones.push(transaccion.clone());
                //Modificar archivo transaccion (agregar transaccion)
                self.cargar_transaccion_al_archivo(&transaccion.clone());

                //Modificar al usuario en el archivo de usuarios_balances. 
                self.cargar_usuarios_al_archivo();

                Ok(())
            }
            false => Err(ErrorIntercambio::UsuarioNoValido),
        }
    }

    pub fn comprar_determinada_criptomoneda(&mut self, monto_fiat: f64, usuario: &mut Usuario, criptomoneda: &Criptomoneda) -> Result<(), ErrorIntercambio> {
        //Check if the user exists.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        //Check if the user is a valid user.
        if !usuario.identidad_validada {
            return Err(ErrorIntercambio::UsuarioNoValido);
        }

        //Check if the user has enough money.
        if usuario.balance_fiat < monto_fiat {
            return Err(ErrorIntercambio::BalanceInsuficiente);
        }

        //Check if the cripto exists.
        if !self.criptomonedas.contains_key(&criptomoneda.nombre) {
            return Err(ErrorIntercambio::CriptoNoEncontrada);
        }

        //If we've come this far it is because the purchase can be made.
        let cotizacion = obtener_cotizacion(&criptomoneda.nombre); 
        usuario.balance_fiat -= monto_fiat; //Descuento el monto_fiat del balance_fiat del usuario.

        //Acredito la cantidad acorde de criptos.
        let cantidad_cripto = monto_fiat/cotizacion;

        *usuario.balance_criptomoneda.entry(criptomoneda.nombre.clone()).or_insert(0.0) += cantidad_cripto;


        /* EXPLICACIÓN:

        entry() = busca si existe la clave, me da acceso para leer/crear la entrada. Retorna un entry enum (occupied, vacant).
        or_insert() = 
            match entry {
                Occupied(entrada) => entrada.get_mut(), // Si existe, devuelve &mut al valor
                Vacant(entrada) => entrada.insert(0.0), // Si no existe, inserta 0.0 y devuelve &mut
            }

        Si "pepe" existe: retorna &mut f64 apuntando al valor actual
        Si "pepe" no existe: crea la entrada con valor 0.0 y retorna &mut f64
        */


        let transaccion = Transaccion {
            fecha: Utc::now(),
            usuario: usuario.clone(),
            tipo: TipoTransaccion::CompraCripto,
            monto_fiat: None,
            criptomoneda: Some(criptomoneda.clone()),
            monto_criptomoneda: Some(cantidad_cripto),
            cotizacion: Some(cotizacion),
            blockchain: None,
            hash: None,
            medio: None,
        };

        self.transacciones.push(transaccion.clone());
        //Modificar archivo transaccion (agregar transaccion)
        self.cargar_transaccion_al_archivo(&transaccion.clone());

        //Modificar al usuario en el archivo de usuarios_balances. 
        self.cargar_usuarios_al_archivo();
        

        Ok(())
    }

    pub fn vender_determinada_criptomoneda(&mut self, usuario: &mut Usuario, criptomoneda: &Criptomoneda, monto_criptomoneda: f64) -> Result<(), ErrorIntercambio> {
        //Check if the user exists.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        //Check if the user is a valid user.
        if !usuario.identidad_validada {
            return Err(ErrorIntercambio::UsuarioNoValido);
        }

        //Check if the cripto exixts.
        if !self.criptomonedas.contains_key(&criptomoneda.nombre) {
            return Err(ErrorIntercambio::CriptoNoEncontrada);
        }

        //Check if the user has enough of that cripto to sell.
        let balance_cripto = usuario.balance_criptomoneda.get(&criptomoneda.nombre).or(Some(&0.0));
        if balance_cripto < Some(&monto_criptomoneda) {
            return Err(ErrorIntercambio::BalanceInsuficiente);
        }

        let cotizacion = obtener_cotizacion(&criptomoneda.nombre);
        let monto_fiat = cotizacion * monto_criptomoneda;

        //Actualizar datos del usuario.
        usuario.balance_fiat += monto_fiat;
        *usuario.balance_criptomoneda.entry(criptomoneda.nombre.clone()).or_insert(0.0) -= monto_criptomoneda; 

        //Creo la transaccion. 
        let transaccion = Transaccion {
            fecha: Utc::now(),
            tipo: TipoTransaccion::VentaCripto,
            usuario: usuario.clone(),
            monto_fiat: None,
            criptomoneda: Some(criptomoneda.clone()),
            monto_criptomoneda: Some(monto_criptomoneda),
            cotizacion: Some(cotizacion),
            blockchain: None,
            hash: None,
            medio: None,
        };

        self.transacciones.push(transaccion.clone());
        //Modificar archivo transaccion (agregar transaccion)
        self.cargar_transaccion_al_archivo(&transaccion.clone());

        //Modificar al usuario en el archivo de usuarios_balances. 
        self.cargar_usuarios_al_archivo();

        Ok(())
    }

    pub fn retirar_criptomoneda_a_blockchain(&mut self, monto_criptomoneda: f64, criptomoneda: &Criptomoneda, blockchain: &Blockchain, usuario: &mut Usuario) -> Result<String, ErrorIntercambio> {
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        if !usuario.identidad_validada {
            return Err(ErrorIntercambio::UsuarioNoValido);
        }

        let balance_cripto = usuario.balance_criptomoneda.get(&criptomoneda.nombre).or(Some(&0.0));
        if balance_cripto < Some(&monto_criptomoneda) {
            return Err(ErrorIntercambio::BalanceInsuficiente);
        }

        //Verificar que la blockchain es soportada por la cripto.
        let cripto = self.criptomonedas.get(&criptomoneda.nombre)
            .ok_or(ErrorIntercambio::CriptoNoEncontrada)?;
        
        let blockchain_soportada = cripto.listado_blockchains.iter()
            .any(|b| b.nombre == blockchain.nombre); //Busco la blockchain dada en el listado de blockchains de la cripto.
        
        if !blockchain_soportada {
            return Err(ErrorIntercambio::BlockchainNoDisponible);
        }

         //Generar hash simulado
         let mut rng = rand::rng();
         let hash = format!("{}{}", blockchain.nombre, rng.random::<u32>());
         
         let cotizacion = obtener_cotizacion(&criptomoneda.nombre);

         *usuario.balance_criptomoneda.get_mut(&criptomoneda.nombre).unwrap() -= monto_criptomoneda; //Descuento la cantidad acorde de esa cripto.

         //Generar transaccion.
         let transaccion = Transaccion {
            fecha: Utc::now(),
            tipo: TipoTransaccion::RetiroCripto,
            usuario: usuario.clone(),
            monto_fiat: None,
            criptomoneda: Some(criptomoneda.clone()),
            monto_criptomoneda: Some(monto_criptomoneda),
            cotizacion: Some(cotizacion),
            blockchain: Some(blockchain.clone()),
            hash: Some(hash.clone()),
            medio: None,
         };

         self.transacciones.push(transaccion.clone());
         //Modificar archivo transaccion (agregar transaccion)
         self.cargar_transaccion_al_archivo(&transaccion.clone());

         //Modificar al usuario en el archivo de usuarios_balances. 
         self.cargar_usuarios_al_archivo();

         Ok(hash)
    }

    pub fn recibir_criptomoneda_de_blockchain(&mut self, monto_criptomoneda: f64, criptomoneda: &Criptomoneda, usuario: &mut Usuario, blockchain: &Blockchain) -> Result<(), ErrorIntercambio> {
        //Check user data.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        if !usuario.identidad_validada {
            return Err(ErrorIntercambio::UsuarioNoValido);
        }

        //Check cripto and blockchain.
        let cripto = self.criptomonedas.get(&criptomoneda.nombre)
            .ok_or(ErrorIntercambio::CriptoNoEncontrada)?; //Si no existe se propaga el error y termina la función.
        
        let blockchain_soportada = cripto.listado_blockchains.iter()
            .any(|b| b.nombre == blockchain.nombre); //Busco la blockchain dada en el listado de blockchains de la cripto.
        
        if !blockchain_soportada {
            return Err(ErrorIntercambio::BlockchainNoDisponible);
        }

        //Acredito la cripto en el balance del usuario.
        let cotizacion = obtener_cotizacion(&criptomoneda.nombre);
        if let Some(balance) = usuario.balance_criptomoneda.get_mut(&criptomoneda.nombre) { //Busco si ya está la cripto en los balances del usuario.
            *balance += monto_criptomoneda / cotizacion; 
        } else {
            // Si la criptomoneda no existe en el balance, la inicializo con el monto dado.
            usuario.balance_criptomoneda.insert(criptomoneda.nombre.clone(), monto_criptomoneda / cotizacion);
        }

        //Genero la transaccion.
        let transaccion = Transaccion {
            fecha: Utc::now(),
            tipo: TipoTransaccion::RecepcionCripto,
            usuario: usuario.clone(),
            criptomoneda: Some(criptomoneda.clone()),
            monto_fiat: None,
            monto_criptomoneda: Some(monto_criptomoneda),
            cotizacion: Some(cotizacion),
            blockchain: Some(blockchain.clone()),
            hash: None,
            medio: None,
        };

        self.transacciones.push(transaccion.clone());
        //Modificar archivo transaccion (agregar transaccion)
        self.cargar_transaccion_al_archivo(&transaccion.clone());

        //Modificar al usuario en el archivo de usuarios_balances. \
        self.cargar_usuarios_al_archivo();

        Ok(())
    }
    
    pub fn retirar_fiat_por_determinado_medio(&mut self, monto_fiat: f64, usuario: &mut Usuario, medio: &Medio) -> Result<(), ErrorIntercambio> {
        //Check user.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        if !usuario.identidad_validada {
            return Err(ErrorIntercambio::UsuarioNoValido);
        }

        //Check if the user has enough money.
        let balance_fiat = usuario.balance_fiat;
        if balance_fiat < monto_fiat {
            return Err(ErrorIntercambio::BalanceInsuficiente);
        }

        //Descontar el monto fiat del monto del usuario.
        usuario.balance_fiat -= monto_fiat;

        //Generar transaccion.
        let transaccion = Transaccion {
            fecha: Utc::now(),
            tipo: TipoTransaccion::RetiroFiat,
            usuario: usuario.clone(),
            monto_fiat: Some(monto_fiat),
            criptomoneda: None,
            monto_criptomoneda: None,
            cotizacion: None,
            blockchain: None,
            hash: None,
            medio: Some(medio.clone()),
        };

        self.transacciones.push(transaccion.clone());
        //Modificar archivo transaccion (agregar transaccion)
        self.cargar_transaccion_al_archivo(&transaccion.clone());
        
        //Modificar al usuario en el archivo de usuarios_balances. 
        self.cargar_usuarios_al_archivo();

        Ok(())
    }




    //Estadísticas.
    pub fn criptomoneda_mas_vendida(&self) -> Option<(String, u32)> { //Return an option with the cripto and the amount of sales.
        let mut auxiliar_vec: Vec<(String, u32)> = Vec::new();

        
        if self.transacciones.is_empty() {
            return None;
        }

        //Ir completando el vector.
        //Solo me fijo en las transacciones cuyo tipo sea VentaCripto.
        self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::VentaCripto).
            for_each(|t| { //Para cada una de ellas.
                if let Some(entry) = auxiliar_vec.iter_mut().find(|(name, _)| *name == t.criptomoneda.as_ref().unwrap().nombre) { //Busco si existen en el vector auxiliar.
                    entry.1 += 1; //Si existen, aumento el contador de ventas asociado a esa cripto.
                } else {
                    auxiliar_vec.push((t.criptomoneda.as_ref().unwrap().nombre.to_string(), 1)); //Si no existe, creo la 'posicion' en el vector.
                }
            });

        auxiliar_vec.iter()
        .max_by_key(|&(_, cantidad)| cantidad)  //Tengo que ver cuál es el máximo de mi vector auxiliar. (Para así saber cuál fue la cripto con más ventas).
        .map(|(nombre, cantidad)| (nombre.clone(), *cantidad)) //El max_by_key me devuelve un Option<&(String, u32)>, yo quiero un Option<(String, u32)>

        /*EXPLANATION: 
        Inside the map closure:

        (nombre, cantidad) destructures the reference: nombre is &String, cantidad is &u32
        nombre.clone() creates an owned String from the &String
        *cantidad dereferences the &u32 to get an owned u32
         */
    }

    pub fn criptomoneda_mas_comprada(&self) -> Option<(String, u32)> { //Return an option with the cripto and the amount of sales.
        let mut auxiliar_vec: Vec<(String, u32)> = Vec::new();
    
        if self.transacciones.is_empty() {
            return None;
        }

        //Ir completando el vector.
        //Solo me fijo en las transacciones cuyo tipo sea CompraCripto.
        self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::CompraCripto)
            .for_each(|t| { //Para cada una de ellas.
                if let Some(entry) = auxiliar_vec.iter_mut().find(|(name, _)| *name == t.criptomoneda.as_ref().unwrap().nombre) { //Busco si existen en el vector auxiliar.
                    entry.1 += 1; //Si existen, aumento el contador de compras asociado a esa cripto.
                } else {
                    auxiliar_vec.push((t.criptomoneda.as_ref().unwrap().nombre.to_string(), 1)); //Si no existe, creo la 'posicion' en el vector.
                }
            });

        auxiliar_vec.iter()
        .max_by_key(|&(_, cantidad)| cantidad)  //Tengo que ver cuál es el máximo de mi vector auxiliar. (Para así saber cuál fue la cripto con más compras).
        .map(|(nombre, cantidad)| (nombre.clone(), *cantidad)) //El max_by_key me devuelve un Option<&(String, u32)>, yo quiero un Option<(String, u32)>
    }

    pub fn crpitomoneda_mas_volumen_venta (&self) -> Option<(String, f64)> {
        let mut auxiliar_vec: Vec<(String, f64)> = Vec::new();

    
        if self.transacciones.is_empty() {
            return None;
        }

        //Ir completando el vector.
        self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::VentaCripto).
            for_each(|t| { //Para cada una de ellas.
                if let Some(entry) = auxiliar_vec.iter_mut().find(|(name, _)| *name == t.criptomoneda.as_ref().unwrap().nombre) { //Busco si existen en el vector auxiliar.
                    entry.1 += t.monto_criptomoneda.unwrap(); //Si existen, aumento el contador asociado a esa cripto.
                    //Sé que es seguro hacer el unwrap. (El tipo de transacción VentaCripto implica que haya algo (Some()) en el monto_criptomoneda).
                } else {
                    auxiliar_vec.push((t.criptomoneda.as_ref().unwrap().nombre.to_string(), t.monto_criptomoneda.unwrap())); //Si no existe, creo la 'posicion' en el vector.
                }
            });

        auxiliar_vec.iter()
        .max_by(|&(_, volumen1), &(_, volumen2)| volumen1.partial_cmp(&volumen2).unwrap())//Tengo que ver cuál es el máximo de mi vector auxiliar. (Para así saber cuál fue la cripto con más volumen de venta).
        .map(|(nombre, volumen)| (nombre.clone(), *volumen))
    }


    pub fn crpitomoneda_mas_volumen_compra (&self) -> Option<(String, f64)> {
        let mut auxiliar_vec: Vec<(String, f64)> = Vec::new();

    
        if self.transacciones.is_empty() {
            return None;
        }

        //Ir completando el vector.
        self.transacciones.iter().filter(|t|t.tipo == TipoTransaccion::CompraCripto).
            for_each(|t| { //Para cada una de ellas.
                if let Some(entry) = auxiliar_vec.iter_mut().find(|(name, _)| *name == t.criptomoneda.as_ref().unwrap().nombre) { //Busco si existen en el vector auxiliar.
                    entry.1 += t.monto_criptomoneda.unwrap(); //Si existen, aumento el contador asociado a esa cripto.
                    //Sé que es seguro hacer el unwrap. (El tipo de transacción CompraCripto implica que haya algo (Some()) en el monto_criptomoneda).
                } else {
                    auxiliar_vec.push((t.criptomoneda.as_ref().unwrap().nombre.to_string(), t.monto_criptomoneda.unwrap())); //Si no existe, creo la 'posicion' en el vector.
                }
            });

        auxiliar_vec.iter()
        .max_by(|&(_, volumen1), &(_, volumen2)| volumen1.partial_cmp(&volumen2).unwrap())//Tengo que ver cuál es el máximo de mi vector auxiliar. (Para así saber cuál fue la cripto con más volumen de compra).
        .map(|(nombre, volumen)| (nombre.clone(), *volumen))
    } 
}


#[cfg(test)]
mod test {
    use core::hash;
    use std::path;

    use super::*;

    fn crear_plataforma() -> PlataformaXYZ {
        let path_transacciones = "src/tp05/archivo_transacciones.txt";
        let path_usuarios_balances = "src/tp05/archivo_usuarios_transacciones.txt";
        let mut plataforma = PlataformaXYZ::new(String::from(path_transacciones), String::from(path_usuarios_balances));



        //Crear usuarios.
        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        let mut user1 = Usuario {
            nombre: "Juan".to_string(),
            apellido: "J".to_string(),
            email: "emailJuan".to_string(),
            dni: 234,
            identidad_validada: true,
            balance_fiat: 2000.0,
            balance_criptomoneda: HashMap::new()
        };

        let mut user2 = Usuario {
            nombre: "Rosita".to_string(),
            apellido: "R".to_string(),
            email: "emailRosita".to_string(),
            dni: 345,
            identidad_validada: true,
            balance_fiat: 7000.0,
            balance_criptomoneda: HashMap::new()
        };

        let mut user3 = Usuario {
            nombre: "Ana".to_string(),
            apellido: "A".to_string(),
            email: "emailAna".to_string(),
            dni: 456,
            identidad_validada: false,
            balance_fiat: 7000.0,
            balance_criptomoneda: HashMap::new()
        };

        plataforma.registrar_usuario(user0);
        plataforma.registrar_usuario(user1);
        plataforma.registrar_usuario(user2);
        plataforma.registrar_usuario(user3);

        // Crear blockchains
        let bitcoin_chain = Blockchain {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
        };

        let ethereum_chain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        let litecoin_chain = Blockchain {
            nombre: "Litecoin".to_string(),
            prefijo: "LTC".to_string(),
        };

        // Crear criptomonedas
        let bitcoin = Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            listado_blockchains: vec![bitcoin_chain],
        };

        let ethereum = Criptomoneda {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
            listado_blockchains: vec![ethereum_chain],
        };

        let litecoin = Criptomoneda {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
            listado_blockchains: vec![litecoin_chain],
        };

        plataforma.registrar_criptomoneda(bitcoin);
        plataforma.registrar_criptomoneda(ethereum);
        plataforma.registrar_criptomoneda(litecoin);

        plataforma
    }


    #[test]
    fn test_ingresar_dinero() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        let mut user3 = Usuario {
            nombre: "Ana".to_string(),
            apellido: "A".to_string(),
            email: "emailAna".to_string(),
            dni: 456,
            identidad_validada: false,
            balance_fiat: 7000.0,
            balance_criptomoneda: HashMap::new()
        };

        //Test with an user that is valid.
        
        //println!("Balance before: {}", &user0.balance_fiat);
        assert!(plataforma.ingresar_dinero(2000.0, &mut user0).is_ok());

        //let usuario = plataforma.usuarios.get(&user0.email);
        //println!("From platform {}", usuario.unwrap().balance_fiat); //Se modifica.

        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0.balance_fiat = updated_user.balance_fiat; // Synchronize user0 with the updated user

        //println!("Balance after: {}", user0.balance_fiat);
        assert_eq!(user0.balance_fiat, 12000.0);


        //Test with an user that is not valid.
        assert!(plataforma.ingresar_dinero(2000.0, &mut user3).is_err()); //Ok.
    }

    #[test]
    fn test_comprar_determinada_moneda() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        let mut user1 = Usuario {
            nombre: "Juan".to_string(),
            apellido: "J".to_string(),
            email: "emailJuan".to_string(),
            dni: 234,
            identidad_validada: true,
            balance_fiat: 2000.0,
            balance_criptomoneda: HashMap::new()
        };

        // Crear blockchains
        let bitcoin_chain = Blockchain {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
        };

        // Crear criptomonedas
        let bitcoin = Criptomoneda { //Esta cripto sí está.
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            listado_blockchains: vec![bitcoin_chain],
        };

        let pepecripto = Criptomoneda { //Esta cripto no.
            nombre: "PepeCripto".to_string(),
            prefijo: "PC".to_string(),
            listado_blockchains: vec![],
        };

        //Compra de una cripto que existe desde un usario que sí puede(user0).
        assert!(plataforma.comprar_determinada_criptomoneda(3000.0, &mut user0, &bitcoin).is_ok()); //Ok.
        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone(); // Synchronize user0 with the updated user
        assert_eq!(user0.balance_criptomoneda.len(), 1); //Tiene la cripto.

        //Compra de una cripto que no existe desde un usuario que sí puede(user0).
        assert!(plataforma.comprar_determinada_criptomoneda(3000.0, &mut user0, &pepecripto).is_err()); //Ok.

        //Compra de una cripto que sí existe desde un usuario que NO puede(user1, por el balance).
        assert!(plataforma.comprar_determinada_criptomoneda(3000.0, &mut user1, &bitcoin).is_err()); //Ok.
    }

    #[test]
    fn test_vender_determinada_criptomeda() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        // Crear blockchains
        let bitcoin_chain = Blockchain {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
        };

        // Crear criptomonedas
        let bitcoin = Criptomoneda { 
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            listado_blockchains: vec![bitcoin_chain],
        };

        plataforma.comprar_determinada_criptomoneda(3000.0, &mut user0, &bitcoin); //Le compro btc.
        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone(); // Synchronize user0 with the updated user

        //Ahora user0 tiene 7000 fiat. Y 0.027963893 btc.

        assert!(plataforma.vender_determinada_criptomoneda(&mut user0, &bitcoin, 0.01).is_ok());
        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone();
        assert_eq!(user0.balance_fiat, 8072.812); //Ok. Se le acreditó el balance fiat de su venta.
    }

    #[test]
    fn test_retirar_criptomoneda_a_blockchain() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        // Crear blockchains
        let bitcoin_chain = Blockchain { //Blockchain válida.
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
        };

        let bitcoin_unvalid_chain = Blockchain { //Blockchain NO válida.
            nombre: "bit".to_string(),
            prefijo: "b".to_string(),
        };

        // Crear criptomonedas
        let bitcoin = Criptomoneda { 
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            listado_blockchains: vec![bitcoin_chain.clone()],
        };


        plataforma.comprar_determinada_criptomoneda(3000.0, &mut user0, &bitcoin); //Le compro btc.
        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone(); // Synchronize user0 with the updated user

        //Ahora user0 tiene 7000 fiat. Y 0.027963893 btc.

        assert!(plataforma.retirar_criptomoneda_a_blockchain(0.01, &bitcoin, &bitcoin_chain, &mut user0).is_ok());

        //let hash = plataforma.retirar_criptomoneda_a_blockchain(0.01, &bitcoin, &bitcoin_chain, &mut user0);
        //println!("{}", hash.unwrap()); 


        //Probar con una blockchain que no existe.
        assert!(plataforma.retirar_criptomoneda_a_blockchain(0.01, &bitcoin, &bitcoin_unvalid_chain, &mut user0).is_err()); //Ok.
    }

    #[test]
    fn test_recibir_criptomoneda_a_blockchain() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        // Crear blockchains
        let bitcoin_chain = Blockchain { 
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
        };

        // Crear criptomonedas
        let bitcoin = Criptomoneda { 
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            listado_blockchains: vec![bitcoin_chain.clone()],
        };

        //Arranca user0 sin criptos. (HM len == 0).
        //println!("Error: {:?}",plataforma.recibir_criptomoneda_de_blockchain(0.01, &bitcoin, &mut user0, &bitcoin_chain) ); 
        assert!(plataforma.recibir_criptomoneda_de_blockchain(0.01, &bitcoin, &mut user0, &bitcoin_chain).is_ok()); //Ok.

        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone(); // Synchronize user0 with the updated user

        assert_eq!(user0.balance_criptomoneda.len(), 1); //Ok.
    }

    #[test]
    fn test_retirar_fiat() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 10000.0,
            balance_criptomoneda: HashMap::new()
        };

        let medio = &Medio::MercadoPago;

        assert!(plataforma.retirar_fiat_por_determinado_medio(5000.0,&mut user0, medio).is_ok()); //Ok.
        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone(); // Synchronize user0 with the updated user
        assert_eq!(user0.balance_fiat, 5000.0); //Ok.
    }

    #[test]
    fn test_estadisticas() {
        let mut plataforma = crear_plataforma();

        let mut user0 = Usuario {
            nombre: "Pepe".to_string(),
            apellido: "P".to_string(),
            email: "emailPepe".to_string(),
            dni: 123,
            identidad_validada: true,
            balance_fiat: 100000.0,
            balance_criptomoneda: HashMap::new()
        };

        //Voy a usar a user0 para haga todas las transacciones.

        // Crear blockchains
        let bitcoin_chain = Blockchain {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
        };

        let ethereum_chain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        let litecoin_chain = Blockchain {
            nombre: "Litecoin".to_string(),
            prefijo: "LTC".to_string(),
        };

        // Crear criptomonedas
        let bitcoin = Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            listado_blockchains: vec![bitcoin_chain],
        };

        let ethereum = Criptomoneda {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
            listado_blockchains: vec![ethereum_chain],
        };

        let litecoin = Criptomoneda {
            nombre: "Litecoin".to_string(),
            prefijo: "LIC".to_string(),
            listado_blockchains: vec![litecoin_chain],
        };

        plataforma.comprar_determinada_criptomoneda(1000.0, &mut user0, &bitcoin);

        plataforma.comprar_determinada_criptomoneda(1000.0, &mut user0, &bitcoin);
        

        plataforma.comprar_determinada_criptomoneda(1000.0, &mut user0, &ethereum);

        plataforma.comprar_determinada_criptomoneda(100.0, &mut user0, &litecoin);
        
        let updated_user = plataforma.usuarios.get(&user0.email).unwrap();
        user0 = updated_user.clone(); // Synchronize user0 with the updated user


        //assert_eq!(user0.balance_criptomoneda.len(), 2);
        //println!("{}", plataforma.transacciones.len());
        assert_eq!(plataforma.criptomoneda_mas_comprada(), Some(("Bitcoin".to_string(), 2))); //Ok.
        assert_eq!(plataforma.crpitomoneda_mas_volumen_compra(), Some(("Ethereum".to_string(), 55.24861878453038))); //Ok.

        plataforma.vender_determinada_criptomoneda(&mut user0, &bitcoin, 0.0001);
        plataforma.vender_determinada_criptomoneda(&mut user0, &bitcoin, 0.0001);
        plataforma.vender_determinada_criptomoneda(&mut user0, &ethereum, 0.0001);

        assert_eq!(plataforma.criptomoneda_mas_vendida(), Some(("Bitcoin".to_string(), 2))); //Ok.
        assert_eq!(plataforma.crpitomoneda_mas_volumen_venta(), Some(("Bitcoin".to_string(), 0.0002))); //Ok.
    }

    #[test]
    fn test_obtener_cotizacion() {
        assert_eq!(obtener_cotizacion("Litecoin"), 94.42);
        assert_eq!(obtener_cotizacion("pepe"), 50.0);

    }

    #[test]
    fn test_cargar_usuarios_al_archivo() {
        let mut plataforma = crear_plataforma();
        
        assert!(plataforma.cargar_usuarios_al_archivo().is_ok()); //Ok. El archivo de modifica bien.

        let mut user_nuevo = Usuario {
            nombre: "Caca".to_string(),
            apellido: "C".to_string(),
            email: "caca@email".to_string(),
            dni: 666,
            identidad_validada: true,
            balance_fiat: 100000.0,
            balance_criptomoneda: HashMap::new()
        };

        plataforma.registrar_usuario(user_nuevo.clone());
        assert!(plataforma.cargar_usuarios_al_archivo().is_ok()); //Se agrega bien al final del archivo. Ok.
    }


}