//# [should_panic]
//#[test]



    pub fn incrementar (number: &mut f64){
        *number = *number + 1.0;
    }

//# [should_panic]
#[test]
fn testear(){
    let mut number:f64 = 0.0;
    incrementar (&mut number);

    assert_eq!(number , 1.0);
}

  