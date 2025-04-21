//# [should_panic]
//# [test]
pub fn act07(){

    fn cant_mayores(numbers:[i32 ; 10] , limite:i32 ) -> u32{

        let mut cant_mayor = 0;
        for number in numbers.iter(){
            if number > &limite{ 
                cant_mayor += 1;
            }
        }

        cant_mayor

    }

    let numeros:[i32 ; 10] = [-5 , 800 , -30, -2 , -1 , 0 , 244 , 9999 , 11 , 12];
    let limite = 0;

    let cant_num_mayores = cant_mayores (numeros , limite);

    assert_eq!(cant_num_mayores , 9); 

}