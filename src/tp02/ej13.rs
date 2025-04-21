//#[should_panic]
//#test]

pub fn act13(){


    fn ordenar_nombres (array: &mut[String; 7]){ //podría usar el mismo nombre como parámetro??
        array.sort();

    }

    let mut names:[String ; 7] = [String::from("Juli") , String::from("Benja") , String::from("Gonza") , String::from("Fio") , String::from("Angi") , String::from("Emi") , String::from("Angel")];

    ordenar_nombres (&mut names);

    assert_eq!(names[0] , "Angel");
}