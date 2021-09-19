pub mod randomized_algos {

    /**
     * This algorithm is used to efficiently solve for modular exponentiation. 
     * This is the general equation for modular exponentiation: x^y mod N
     */
    pub fn mod_exponent(base: f64, exp: f64, modulo: f64) -> f64 {
        if exp == 0.0 {return 1.0;}                 //base case
        let y = (exp / 2.0).floor();
        let z = mod_exponent(base, y, modulo);      //recurse     
        if exp % 2.0 == 0.0 {                       //fi exponent is even
            z.powf(2.0) % modulo
        } else {
            (base * z).powf(2.0) % modulo
        }
    }

    pub fn euclid_gcd(){


    }

    pub fn euclid_gcd_ext(){}

    pub fn simple_primality(){}

    pub fn carmichael_nums(){}
}