use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rand::Rng;

#[derive(Clone, Debug)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: u32,
    identidad_validada: bool,
    balance_fiat: f64,
    balance_criptomoneda: HashMap<String, f64>, //Nombre_cripto -> cantidad.
}

#[derive(Clone, Debug)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Clone, Debug)]
struct Criptomoneda { 
    nombre: String,
    prefijo: String,
    listado_blockchains: Vec<Blockchain>,
}

#[derive(Clone, Debug)]
enum TipoTransaccion {
    IngresoFiat,
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    RetiroFiat,
}

#[derive(Clone, Debug)]
enum Medio {
    MercadoPago,
    TransferenciaBancaria,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct PlataformaXYZ {
    usuarios: HashMap<String, Usuario>, //Email -> usuario.
    criptomonedas: HashMap<String, Criptomoneda>, //Nombre_cripto -> cripto.
    transacciones: Vec<Transaccion>,
}

//Custom errors.
enum ErrorIntercambio {
    UsuarioNoValido,
    BalanceInsuficiente,
    CriptoNoEncontrada,
    BlockchainNoDisponible,
    UsuarioNoEncontrado,
}


impl PlataformaXYZ {
    fn new() -> Self {
        PlataformaXYZ { usuarios: HashMap::new(), criptomonedas: HashMap::new(), transacciones:Vec::new() }
    }

    //Obtener cotizacion.
    fn obtener_cotizacion(&self, cripto_nombre: &str) -> f64  { //En dólares. 
        match cripto_nombre {
            "Bitcoin" => 107281.20,
            "ETC" => 18.10,
            "Litecoin" => 94.42,
            _ => 50.0,
        }
    }

    //Registrar usuario.
    fn registrar_usuario(&mut self, usuario: Usuario) { //No lo paso como &Usuario porque lo que le voy a mandar va a ser un clone (una copia). Is that okay???
        self.usuarios.insert(usuario.email.clone(), usuario);
    }

    fn registrar_criptomoneda(&mut self, criptomoneda: Criptomoneda) {
        self.criptomonedas.insert(criptomoneda.nombre.clone(), criptomoneda);
    }

    fn ingresar_dinero(&mut self, monto_fiat: f64, usuario: Usuario) -> Result<(), ErrorIntercambio> {
        //Verify that the user exists and it is valided.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        match usuario.identidad_validada {
            true => {
                //Acredito el monto fiat.
                usuario.balance_fiat += monto_fiat;

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
                self.transacciones.push(transaccion);
                Ok(())
            }
            false => Err(ErrorIntercambio::UsuarioNoValido),
        }
    }

    fn comprar_determinada_criptomoneda(&mut self, monto_fiat: f64, usuario: Usuario, criptomoneda: &Criptomoneda) -> Result<(), ErrorIntercambio> {
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
        let cotizacion = self.obtener_cotizacion(&criptomoneda.nombre); //How can I fix this???
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

        self.transacciones.push(transaccion);
        Ok(())
    }

    fn vender_determinada_criptomoneda(&mut self, usuario: &Usuario, criptomoneda: &Criptomoneda, monto_criptomoneda: f64) -> Result<(), ErrorIntercambio> {
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

        let cotizacion = self.obtener_cotizacion(&criptomoneda.nombre);
        let monto_fiat = cotizacion * monto_criptomoneda;

        

        //Actualizar datos del usuario.
        usuario.balance_fiat += monto_fiat;
        *usuario.balance_criptomoneda.entry(criptomoneda.nombre.clone()).or_insert(0.0) -= monto_criptomoneda/cotizacion; //preg if this is okay (Cantidad_cripto = monto_criptomoneda/cotizacion)

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

        self.transacciones.push(transaccion);
        Ok(())
    }

    fn retirar_criptomoneda_a_blockchain(&mut self, monto_criptomoneda: f64, criptomoneda: &Criptomoneda, blockchain: &Blockchain, usuario: &Usuario) -> Result<String, ErrorIntercambio> {
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
         
         let cotizacion = self.obtener_cotizacion(&criptomoneda.nombre);

         *usuario.balance_criptomoneda.get_mut(&criptomoneda.nombre).unwrap() -= monto_criptomoneda/cotizacion; //Descuento la cantidad acorde de esa cripto.

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

         self.transacciones.push(transaccion);
         Ok(hash)
    }

    fn recibir_criptomoneda_de_blockchain(&mut self, monto_criptomoneda: f64, criptomoneda: &Criptomoneda, usuario: &Usuario, blockchain: &Blockchain) -> Result<(), ErrorIntercambio> {
        //Check user data.
        let usuario = self.usuarios.get_mut(&usuario.email)
            .ok_or(ErrorIntercambio::UsuarioNoEncontrado)?;

        if !usuario.identidad_validada {
            return Err(ErrorIntercambio::UsuarioNoValido);
        }

        //Check cripto and blockchain.
        let cripto = self.criptomonedas.get(&criptomoneda.nombre)
            .ok_or(ErrorIntercambio::CriptoNoEncontrada)?;
        
        let blockchain_soportada = cripto.listado_blockchains.iter()
            .any(|b| b.nombre == blockchain.nombre); //Busco la blockchain dada en el listado de blockchains de la cripto.
        
        if !blockchain_soportada {
            return Err(ErrorIntercambio::BlockchainNoDisponible);
        }

        //Acredito la cripto en el balance del usuario.
        let cotizacion = self.obtener_cotizacion(&criptomoneda.nombre);
        *usuario.balance_criptomoneda.get_mut(&criptomoneda.nombre).unwrap() += monto_criptomoneda/cotizacion;

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

        self.transacciones.push(transaccion);
        Ok(())
    }
    
    fn retirar_fiat_por_determinado_medio(&mut self, monto_fiat: f64, usuario: &Usuario, medio: &Medio) -> Result<(), ErrorIntercambio> {
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

        self.transacciones.push(transaccion);
        Ok(())
    }

    
}