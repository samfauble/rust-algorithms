extern crate rand;
extern crate bit_vec;
use bit_vec::*;
use rand::*;
use std::convert::TryInto;
use crate::randomized::randomized_algos::*;

/**
 * For creation of key roots p and q
 */
pub fn generate_rand_prime() -> u128 {
    let mut num_bits: BitVec = BitVec::new();
    
    //populate bitvecs with random bools
    for _i in 1..128 {
        let r: bool = random();
        num_bits.push(r);
    }

    //convert bitvecs to nums
    let num_as_num = u128::from_be_bytes(num_bits.to_bytes().try_into().unwrap());
    
    //determine if pub and priv key are prime
    let num_is_prime = simple_primality(&num_as_num, 5);
    
    if num_is_prime {
        num_as_num
    } else {
        generate_rand_prime()
    }
}

//pub key (N, e) 
//priv key d (congruent to e^-1 % (p-1)(q-1))
pub fn gen_key_pair(p: u128, q: u128) -> ((u128, u128), u128) {
    let mut found_rel_prime = false;
    let mut e = 0;                                      //public key
    let mut d = 0;                                      //private key
    let pub_key: (u128, u128);
    let rel_prime = (p - 1)*(q - 1);                    //mod value

    while !found_rel_prime {
        e = rand::thread_rng().gen_range(0..u128::MAX);
        let res = euclid_gcd_ext(e, rel_prime);
        //if rel_prime and e are relatively prime
        if res.gcd == 1 {
            found_rel_prime = true;
            d = res.alpha;
        }
    }

    pub_key = (rel_prime, e);

    (pub_key, d)
}

//m^e % N = encrypted message
pub fn encrypt(message: &str, pub_key_reciever: (u128, u128)) -> u128{
    let message_as_int = message.to_string().parse::<u128>().unwrap();
    mod_exponent(message_as_int as f64, pub_key_reciever.1 as f64, pub_key_reciever.0 as f64) as u128
}

//y^d % N = decrypted message
pub fn decrypt(encrypted: u128, pub_key: (u128, u128), priv_key: u128) -> u128{
    mod_exponent(encrypted as f64, priv_key as f64, pub_key.0 as f64) as u128
}
