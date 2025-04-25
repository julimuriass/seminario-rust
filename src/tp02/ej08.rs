




    pub fn sumar_arreglos(array1:[f64 ; 2] , array2:[f64 ; 2]) -> [f64 ; 2]{

        let mut results:[f64 ; 2]= [0.0 , 0.0];

        for n in 0..array1.len(){
            results[n] = array1[n] + array2[n];
        }

        results
    }

//# [should_panic]
# [test]
fn testear(){
    let numbers1:[f64 ; 2]= [0.0 , 25.2];
    let numbers2:[f64 ; 2]= [-2.4 , 3.1];

    let sums:[f64 ; 2]= sumar_arreglos(numbers1 , numbers2);

    assert_eq!(sums[0] , -2.4);
    assert_eq!(sums[1] , 28.3);
}
    

