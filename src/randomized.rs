pub mod randomized_algos {
    extern crate rand;
    use rand::{Rng, thread_rng};
    use std::ops::Range;

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
        if exp % 2.0 == 0.0 {                       //if exponent is even
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

    /**
     * Euclid's extended algorithm (EEA) is used to calculate GCD and multiplicative inverses.
     * If GCD(x, y) = 1 where (x % y), then EEA can give us the inverse (x^-1 % y).
     * In the result, alpha % y = x^-1 % y. 
     */
    pub fn euclid_gcd_ext(x: u128, y: u128) -> ExtEuclidRes{
        if y == 0 {
            ExtEuclidRes{alpha: 1, beta: 0, gcd: x}                         //base case
        } else {
            let res = euclid_gcd_ext(y, x % y);                             //recurse
            let beta_prime = res.alpha - ((x/y) * res.beta);                //calc beta'
            ExtEuclidRes{alpha: res.alpha, beta: beta_prime, gcd: res.gcd}  //result
        }
    }

    /**
     * The simple primality algorithm is used to test for whether a value x is prime.
     * x is prime if it is relatively prime with all numbers from 1 - (x-1),
     * so this algorithm tests for relative primality of x with a random value r.
     * 
     * If the two values aren't relatively prime, then r acts as a Fermat witness to
     * the fact that x is not prime. If they are relatively prime, x might be prime,
     * but we can't be certain.
     * 
     * For this implementation, true is returned if x might be prime.
     */
    pub fn simple_primality(x: u32, rounds: usize) -> bool{
        let mut res: bool = true;
        
        for _i in 0..rounds {
            let r = thread_rng().gen_range(1..x);
            let z = mod_exponent(x as f64, (r-1) as f64, r as f64);
            if z as u64 != (1 % r) as u64 {
                res = false;
                break;
            }
        }

        res
    }

    pub fn carmichael_nums() {}
}