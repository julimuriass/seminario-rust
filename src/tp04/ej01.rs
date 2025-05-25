pub trait Primo {
    fn soy_primo (&self) -> bool;
}

impl Primo for i32 {

    fn soy_primo(&self) -> bool {
        if *self < 2 {
            return false;
        }
        for number in 2..=(*self as f64).sqrt() as i32 {
            if self % number == 0 {
                return false;
            }
        }
        true
    }

}

pub fn contar_primos(numeros: &Vec<i32>) -> u32 {    
    //Preg si no pasa nada al usar el .count(), porque el count me consume el vector ????!!!!!
    //&&i32 ????
    numeros.iter().filter(|x| x.soy_primo()).count() as u32
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testar_contar_primos() {
        let numbers:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];

        assert_eq!(contar_primos(&numbers), 4);
    }

}
