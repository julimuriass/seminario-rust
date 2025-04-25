


    pub fn cantidad_de_cadenas_mayor_a (array:[String ; 4] , limite: u32) -> u32{
        let mut cant_result:u32 = 0;

        for element in array.iter(){
            if element.len() as u32 > limite{
                cant_result += 1;
            }
        }

        cant_result

    }

//#[should_panic]
#[test]
fn testear(){
    let strings:[String ; 4] = [String::from("Hello") , String::from("Abracadabra") , String::from("Eye") , String::from("Banana")];
    let limite:u32 = 4;

    let cant_cumplen = cantidad_de_cadenas_mayor_a(strings , limite);

    assert_eq!(cant_cumplen , 3);
}

    

