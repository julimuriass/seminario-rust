


    pub fn cantidad_en_rango (array:[i32 ; 10] , inferior: i32 , superior: i32) -> u32{
        let mut cant_num:u32 = 0;

        for number in array.iter(){
            if &inferior <= number && number <= &superior{
                cant_num += 1;
            }
        }

        cant_num
    }

#[should_panic]
#[test]
fn testear(){
    let numbers:[i32 ; 10]= [0,1,2,3,4,5,6,7,8,9];
    let inferior = 1;
    let superior = 5;

    let cant_numeros_entre = cantidad_en_rango(numbers , inferior , superior);

    assert_eq!(cant_numeros_entre , 9);
}
   
