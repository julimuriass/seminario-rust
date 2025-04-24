//# [should_panic]
//# [test]

pub fn act06(){

    fn longitud_de_cadenas(strings:[String ; 3]) -> [u32 ; 3]{

        let mut longitudes:[u32 ; 3] = [0 , 0 , 0];
        for n in 0..strings.len(){
            longitudes[n] = strings[n].len() as u32;
        }

        longitudes

    }
      


    let arreglo_strings:[String ; 3] = [String::from("Hello") , String::from("world") , String::from("!")]; //preg if okay
    let output:[u32 ; 3] = longitud_de_cadenas (arreglo_strings);

    assert_eq!(output[0] , 8); 

}