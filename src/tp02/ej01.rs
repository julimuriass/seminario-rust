
#[test] //??????
pub fn es_par(){

    fn es_par_check(numero: i32) -> bool {
        numero % 2 == 0
    }

   
    //#[should_panic] como lo puedo implementar??
    
    let number:i32 = 2;
    let par:bool = es_par_check(number);
    assert_eq!(par, true);

    println!("El numero {} es par: {}" , number , par);

}