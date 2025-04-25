


    pub fn cant_impar (numbers:[i32 ; 10]) -> u32{

        let mut impares = 0;
        for number in numbers.iter(){
            if number % 2 != 0{
                impares += 1;
            }
        }

        impares
    }


# [should_panic]
# [test]
fn helper (){
    let numeros:[i32 ; 10] = [1,2,3,4,5,6,7,8,9,10];
    let numeros_impares = cant_impar(numeros);

    assert_eq!(numeros_impares , 9);

}

    
