pub mod dc_algos {
    extern crate bit_vec;
    extern crate median;
    extern crate imageproc;
    extern crate num;
    use num::complex::Complex;
    use imageproc::hough::PolarLine;
    use std::f64::consts::PI;
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

    /**
     * given a coefficient_vec (a0, a1,...,an-1) for polynomial A(x) = a0 + a1x +...+ an-1 x^n-1
     * Assumptions: n = 2^k
     */
    pub fn fft(coefficient_vec: Vec<i32>, omega: (f64, f64)) -> Vec<(f64, f64)> {
        let omega_to_exp = |o: &(f64, f64), exp: u32| -> (f64, f64) {
            let mut sol = *o;
            for _e in 1..exp {
                sol.0 *= o.0;
                sol.1 += o.1;
            }
            sol
        };

        let eval_poly = |x: &(f64, f64)| -> Vec<(f64, f64)> {
            let mut agg = vec![];
            for i in 0..coefficient_vec.len() - 1 {
                let mut new_omega = omega_to_exp(x, i as u32);
                new_omega.0 *= coefficient_vec[i] as f64;
                agg.push(new_omega);
            }
            agg
        };

        if coefficient_vec.len() == 1 {return eval_poly(&omega);}

        let mut even = vec![];
        let mut odd = vec![];
        let mut toggle = true;

        coefficient_vec.iter().for_each(|val| {
            if toggle {
                even.push(*val);
            } else {
                odd.push(*val);
            }
            toggle = !toggle;
        });

        let next_omega = omega_to_exp(&omega, 2);

        let evens = fft(even, next_omega);
        let odds = fft(odd, next_omega);

        let mut first_half = vec![];
        let mut second_half = vec![];

        for j in 0..(coefficient_vec.len() / 2) - 1 {
            let j_omega = omega_to_exp(&omega, j as u32);
            let product = ((j_omega.0 * odds[j].0), (j_omega.1 + odds[j].1));
            let even = Complex::from_polar(evens[j].0, evens[j].1);
            let c_product = Complex::from_polar(product.0, product.1);
            
            let mut c_first = Complex::new(0.0, 0.0);
            let mut c_second = Complex::new(0.0, 0.0);
            c_first.re = even.re + c_product.re;
            c_first.im = even.im + c_product.im;
            c_second.re = even.re - c_product.re;
            c_second.im = even.im - c_product.im;

            first_half.push(Complex::to_polar(c_first));
            second_half.push(Complex::to_polar(c_second));
        }
        
        first_half.append(&mut second_half);
        first_half
    }
}