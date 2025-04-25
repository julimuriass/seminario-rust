

    
    pub fn suma_pares(arreglo:[i32; 10]) -> i32{
        
        let mut suma_p:i32 = 0;
        for number in arreglo.iter(){
            if number % 2 == 0 {
                suma_p += number;
            }
        }

        suma_p
    }

    //#[test] 
    fn testear (){
        let numeros:[i32; 10] = [1,2,3,4,5,6,7,8,9,10];
        let resultado = suma_pares(numeros);

        assert_eq!(resultado, 30);

    }
    


