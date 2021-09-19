pub mod dc_algos {
    extern crate bit_vec;
    use bit_vec::BitVec;
    use std::convert::TryInto;


    /**
     * A more efficient way to multiply n-bit integers x and y
     * Assume x and y are a power of 2 and both have same n-bits
     */
    pub fn gauss_mult(x: u128, y: u128)  -> u128{
        //Convert x and y to bit array
        //split bit array in half
        let x_bytes = x.to_be_bytes();
        let mut xl = BitVec::from_bytes(&x_bytes);
        let xr = xl.split_off(xl.len()/2);
        let num_bit = xl.len() + xr.len();

        let y_bytes = y.to_be_bytes();
        let mut yl = BitVec::from_bytes(&y_bytes);
        let yr = yl.split_off(yl.len()/2);

        //Convert the resulting four bit array halves back in to ints
        let xl_num = u128::from_be_bytes(xl.to_bytes().try_into().unwrap());
        let xr_num = u128::from_be_bytes(xr.to_bytes().try_into().unwrap());
        let yr_num = u128::from_be_bytes(yr.to_bytes().try_into().unwrap());
        let yl_num = u128::from_be_bytes(yl.to_bytes().try_into().unwrap());

        //Recurse
        let a = gauss_mult(xl_num, yl_num);
        let b = gauss_mult(xr_num, yr_num);
        let c = gauss_mult(xl_num + xr_num, yl_num + yr_num);

        //Multiply together
        //2^n * a + 2^(n/2) * (c - a -b) + b
        (2_u128.pow(num_bit as u32) * a) + (2_u128.pow(num_bit as u32 / 2) * (c - a - b)) + b
    }
}