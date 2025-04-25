
    pub fn es_par(numero: i32) -> bool {
        numero % 2 == 0
    }

    #[test]
    //#[should_panic] 
    fn probar (){
        let number:i32 = 2;
        let par:bool = es_par(number);
        assert_eq!(par, true);
    }
   
