



    pub fn multiplicar_valores ( array: &mut [i32 ; 5] , factor: i32){
        
        for number in array.iter_mut(){
            *number = *number * factor; 
        }

    }

//#[should_panic]
#[test]
fn testear(){
    let mut numbers:[i32 ; 5] = [1 , 2 , 3 , 4 , 5];
    let factor: i32 = -1;

    multiplicar_valores(&mut numbers , factor); 

    assert_eq!(numbers[0] , -1);
}

    
