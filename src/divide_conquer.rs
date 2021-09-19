pub mod dc_algos {
    extern crate bit_vec;
    extern crate median;
    use bit_vec::BitVec;
    use std::convert::TryInto;
    use median::heap::*;


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

    /**
     * Find k-th smallest value in given array in O(n) time
     */
    pub fn fast_select(arr: Vec<u64>, k: usize) -> u64 {
        let num_groups = (((arr.len() / 5) as f32).ceil()) as u32; 
        let mut groups: Vec<Vec<u64>> = Vec::new();
        let mut medians: Vec<u64> = Vec::new();
        let mut counter = 0;

        //Find an optimal pivot point
        //divide arr into subarrays of 5 sorted elems
        for _i in 1..num_groups {
            let mut g: Vec<u64> = Vec::new();
            for j in 0..4 {
                g.push(arr[counter + j]);
            }
            g.sort();
            groups.push(g);
            counter += 5; 
        }

        //find the median of each subarray
        for subarr in groups {
            let mut filter = median::Filter::<u64>::new(subarr.len());
            for val in subarr {
                filter.consume(val);
            }
            let med = filter.median();
            medians.push(med);
        }

        //recurse to find the optimal pivot
        let pivot = fast_select(medians, arr.len()/10); 

        //Find the k-th smallest value in arr
        let mut answer = 0;
        for elem in arr {
            let mut bigger = vec![];
            let mut  equal = vec![];
            let mut smaller = vec![];

            if elem < pivot {smaller.push(elem);}
            if elem == pivot {equal.push(elem);}
            if elem > pivot {bigger.push(elem);}

            if k <= smaller.len() {
                return fast_select(smaller, k);
            } else if k > (smaller.len() + equal.len()) {
                return fast_select(bigger, k);
            }else{
                answer = pivot;
            } 
        }
        answer
    }
}