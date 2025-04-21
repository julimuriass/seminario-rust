//# [should_panic]
//#[test]
pub fn act12(){

    fn reemplazar_pares (array:&mut[i32 ; 10]){

        for number in array.iter_mut(){
            if *number % 2 == 0 {
                *number = -1;
            }
        }
    }


    let mut numbers:[i32 ; 10] = [1,2,3,4,5,6,7,8,9,10];
    reemplazar_pares (&mut numbers);

    assert_eq!(numbers[1] , -1);

}