//mod tp02;
//mod tp03;
//mod tp04;
mod tp05;


use std::io::stdin;
use core::panic;
const NUMERO:i32 = 10; //ejercicio 7
const WORD: &str = "Abracadabra"; //ejercicio 8

fn main() { 
    //Práctica 1:
    
    //ejercicio 1--------------------------------
    let fp_number  = 2.0;
    println! ("Write another number: ");
    let mut number = String::new();
    stdin().read_line(& mut number).expect("error");
    let number_okay:f64 = number.trim().parse().expect("error2");

    println! ("first operation * , {}" , fp_number * number_okay);
    println! ("second operation / , {}" , number_okay / fp_number);
    println! ("third operation + , {}" , fp_number + number_okay);
    println! ("fourth operation - , {}" , number_okay - fp_number);


    //ejercicio 2------------------------------------
    let integer_number:u32 = 1234567890;
    let hex_string = format!("{:X}" , integer_number);
    println!("{}", hex_string);


    //ejercicio 3---------------------------------
    let t:bool = true; //defino variable booleana

    let mut var_ingresada=String::new(); //el dato tiene que ser de tipo string para leerlo por teclado

    println!("Ingresa un valor booleano (true or false");


   //leo el dato
    stdin().read_line(&mut var_ingresada)
    .expect("error"); 

    //convierto el dato a un booleano
    let mut var_ingresada: bool = match var_ingresada.trim().parse(){
        Ok(bool) => bool,
        Err(_) => {
            eprintln!("error");
            return;
        }
    };

    let mut var_ingresada = var_ingresada && t;
    println!("AND {} ", var_ingresada);

    let mut var_ingresada = var_ingresada || t;
    println!("OR {} ", var_ingresada);



    //ejercicio4 ----------------------------------------
    let tupla:(String , i32 , bool) = ("let's get rusty".to_string() , 10 , false);
    println!("{} , {} , {}" , tupla.0, tupla.1 , tupla.2);



    //ejercicio 5 ----------------------------------------
    let mut chain = String::from("Hello ");

    let mut chain_ingresada = String::new();

    println!("Ingresa una cadena");

    //leo el dato
    stdin().read_line(&mut chain_ingresada).expect("error");

    //concateno ambas cadenas
    chain.push_str(&*chain_ingresada);

    let chain = chain.to_string();
    let chain = chain.to_ascii_uppercase();

    println!("{}", chain);


    //ejercicio 6 ------------------------------------------
    let mut number :u32 = 2;
    let mut number_ingresado = String::new();

    println!("Ingresa un número");

    stdin().read_line(&mut number_ingresado).
    expect("error");

    let mut number_ingresado:u32 = match number_ingresado.trim().parse(){
        Ok(u32) => u32,
        Err(_) => {
            eprintln!("error");
            return;
        }
    };

    let mut number = number + number_ingresado;
    println!("{}", number.pow(2));


    //ejercicio 7 ------------------------------------------
    let mut arreglo:[i32 ; 6] = [10 , 4 , 77 , 6 , 8 , 0];
    
    for number in arreglo.iter_mut(){
        *number = match number{
        &mut number => number * NUMERO,
        }
     }
    
    for index in 0..arreglo.len() {
        println!("{}", arreglo[index]);
    }



    //ejercicio 8 --------------------------------------
    println!("Ingrese una letra para ver la cantidad de veces que ésta aparece en la constante definida");

    let mut caracter = String::from (" ");


    //leo el caracter
    stdin().read_line(&mut caracter)
    .expect("error");


    //convierto el string que leí a caracter
    let mut caracter: char = caracter.trim().parse()
    .expect("error");

    let mut cantidad: u32 = 0;
    for letra in WORD.chars(){
        if caracter.to_ascii_lowercase() == letra.to_ascii_lowercase(){
            cantidad = cantidad + 1;
        }
    }

    println!( "La cantidad de veces que aparece el caracter {} es : {}", caracter , cantidad);



    //ejercicio 9 -------------------------------------
    let numeros : [i32 ; 5] = [0 , 300 , -100 , 10 , 9];

    let mut suma_total:i32 = 0;

    for numero in 0..numeros.len(){
        suma_total += numeros[numero];
    }

    println!("La suma total de los números es {}" , suma_total);



    //ejercicio 10 ---------------------------------------------
    let numeros1: [i32 ; 5] = [10 , 20 , 30 , 40 , 50];
    let numeros2: [i32 ; 5] =  [1 , 2 , 3 , 4 , 5];

    let mut numeros_suma: [i32 ; 5] = [0, 0, 0, 0, 0];

    
    for number in 0..5{
        let mut suma_auxiliar: i32 = 0;
        suma_auxiliar = numeros1[number] + numeros2[number];

        numeros_suma[number] = suma_auxiliar;
    }

    for elemento in 0..numeros_suma.len(){
        println!("{}" , numeros_suma[elemento]);
    }



    //ejercicio 11 ---------------------------------------
    let cadenas:[&str ; 5] = ["Hello" , "Bye" , "How are you?" , "Pepe" , "Good"];

    println!("Ingrese una cadena");
    let mut cadena_ingresada = String::new();


    stdin().read_line(&mut cadena_ingresada)
    .expect("Error :/");

    let cadena_ingresada = cadena_ingresada.trim();

    if cadenas.contains(&cadena_ingresada){
        println!("Está! :D");
    } else {
        println!("No está :/");
    }



    //ejercicio 12-----------------------------------
    let numeros: [i32 ; 4] = [0 , -10 , -20 , 100];
    let tupla: (String , [i32 ; 4]) = ("Hello pepe".to_string() , numeros);

    println!("La cadena de la tupla dice: {}" , tupla.0);

    let mut suma_total: i32 = 0;
    for number in 0..tupla.1.len(){
        suma_total += tupla.1[number];

    }

    println!("{}" , suma_total);
    



}