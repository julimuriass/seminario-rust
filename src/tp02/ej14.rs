//# [should_panic]
//#[test]

pub fn act14(){

    fn incrementar (number: &mut f64){
        *number = *number + 1.0;
    }


    let mut number:f64 = 0.0;
    incrementar (&mut number);

    assert_eq!(number , 1.0);
}