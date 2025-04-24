pub fn act05(){

    fn duplicar_valores(numbers:[f64 ; 5]) -> [f64 ; 5]{

        let mut new_array :[f64 ; 5] = [0.0,0.0,0.0,0.0,0.0];

        for n in 0 .. numbers.len(){
            new_array[n] = numbers[n] * 2.0;

        }

        new_array

    }

    fn imprimir_arreglo(numbers: [f64 ; 5]){
        for number in numbers.iter(){
            println! ("{}" , number);
        }
    }


    let numeros:[f64 ; 5] = [0.0 , 1.1 , 2.2 , 3.3 , 4.4];
    let doble_numeros:[f64 ; 5] = duplicar_valores(numeros);

    imprimir_arreglo(doble_numeros); //preg if okay???? pq no se podía usar el println! mod


}