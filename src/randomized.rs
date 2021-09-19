pub mod randomized_algos {
    extern crate rand;
    use rand::{thread_rng};

    pub struct ExtEuclidRes {
        alpha: u128,
        beta: u128,
        gcd: u128
    }

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

    /**
     * Euclid's rule, which is the basis for this simple algorithm, is the following:
     * The greatest common divisor (GCD) of a given x and y where x >= y >= 0
     * is x modulo y and y. Written as an equation: GCD(x, y) = GCD(x % y, y).
     * 
     * If the GCD(x, y) = 1, it means that x and y are relatively prime to one another
     */
    pub fn euclid_gcd(x: u128, y: u128) -> u128{
        if y == 0 {
            x                   //base case
        } else {
            euclid_gcd(y, x % y)
        }
    }

    pub fn euclid_gcd_ext(x: u128, y: u128) -> ExtEuclidRes{
        if y == 0 {
            ExtEuclidRes{alpha: 1, beta: 0, gcd: x}
        } else {
            let res = euclid_gcd_ext(y, x % y);
            let beta_prime = res.alpha - ((x/y) * res.beta);
            ExtEuclidRes{alpha: res.alpha, beta: beta_prime, gcd: res.gcd}
        }
    }

    pub fn simple_primality(x: u128, rounds: usize) -> bool{
        let res: bool = true;
        
        for _i in 0..rounds {
            let r = rand::random::<u128>();
            let z = x.pow(r);
            let modulo = 1 % r;
            if z != modulo as u128 {
                res = false;
                break;
            }
        }

        return res;
    }

    pub fn carmichael_nums(){}
}