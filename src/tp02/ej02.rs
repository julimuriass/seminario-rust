
    
    pub fn es_primo (numero : i32) -> bool{

        if numero > 1 {

            //check if primo
            if (numero % numero == 0) & (numero % 1 == 0){ 
                if numero % (numero/2) != 0 {
                    true
                }

                else{
                    false
                }
            } else {
                false
            }
        }

        else{
            false
        }

    }

    #[test]
    fn testear(){
       let number:i32 = 1;
        let primo:bool = es_primo(number);

         assert_eq!(primo, false);
    }
    


