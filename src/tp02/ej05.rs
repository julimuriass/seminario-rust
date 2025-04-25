

    fn duplicar_valores(numbers:[f64 ; 5]) -> [f64 ; 5]{

        let mut new_array :[f64 ; 5] = [0.0,0.0,0.0,0.0,0.0];

        for n in 0 .. numbers.len(){
            new_array[n] = numbers[n] * 2.0;

        }

        new_array

    }


 //# [should_panic]
 # [test]
fn helper(){
    let numeros:[f64 ; 5] = [0.0 , 1.1 , 2.2 , 3.3 , 4.4];
    let doble_numeros:[f64 ; 5] = duplicar_valores(numeros);

    assert_eq!(doble_numeros[0] , 0.0)
}

