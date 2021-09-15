
pub mod dynamic_algos {

    pub struct Item <T> {
        value: T,
        weight: T
    }

    impl Item<i32> {
        pub fn new(val: i32, w: i32) -> Item<i32> {
            Item::<i32> {
                value: val,
                weight: w
            }
        }
    }

    /**
    * Calculates the longest increasing subsequence (lis) of arg arr
    * 
    * lis definition - a sequential set of array elems: 
    * - whose values are larger than that of the preceding elem 
    * - whose index is larger than that of the preceding elem
    * - that need not be consecutive (intermediate elems in original array can be dropped)
    * 
    * Consumes an array of itegers
    * Returns the length of the longest lis in the array
    * Runs in O(n^2) time
    */
    pub fn lis (arr: Vec<i32>) -> i32 {
        //edge cases
        if arr.len() == 1 {
            return 1;
        }

        if arr.len() == 0 {
            return 0;
        }

        let mut i: usize = 0; //index for arr
        let mut j: usize = 0; //index for lis_arr
        let mut lis_arr = Vec::new(); // list of largest lis lengths at arr[i]

        //populate lis_arr with with same number of elems as arr
        // each elem = 1
        while i < arr.len() {
            lis_arr.push(1);
            i += 1;
        }

        i = 1; //reset arr index

        //iterate through all elems of arr
        while i < arr.len() {
            j = 0; //reset lis_arr index

            //iterate through subarray arr[a0..a(i-1)]
            //arr[i] = current arr value iterated over
            //arr[j] = current subarray value iterated over
            //lis_arr[i] = lis length value corresponding with arr[i]
            //lis_arr[j] = lis length value corresponding with arr[j]
            while j < i {
                if arr[i] > arr[j] && lis_arr[i] < lis_arr[j] + 1 {
                    lis_arr[i] = lis_arr[j] + 1;
                }

                j += 1;
            }

            i += 1;
        }

        //find the largest length in lis_arr
        let mut solution = 1;
        for elem in lis_arr.iter() {
            if *elem > solution {
                solution = *elem;
            }
        }

        solution
    }


    /**
     * Returns the length of the longest common subsequence (LCS)
     * LCS - The longest shared series of characters between the two strings
     * Input: Two strings
     * Output: u32 int
     */
    pub fn lcs(string_1: String, string_2: String) -> u32 {
        let mut lcs_table: Vec<Vec<u32>> = Vec::new(); //a 2D array for vals
        let mut i_arr: Vec<u32> = Vec::new(); //a row
        let mut i = 0; // str1 index
        let mut j = 0; // str2 index

        //populate 2D array with zeros
        while i < string_1.len() {
            i_arr.push(0);
            i += 1;
        }

        while j < string_2.len() {
            let list = &i_arr;
            let v = list.to_vec();
            lcs_table.push(v);
            j += 1;
        }

        //reset indexes
        i = 0;
        j = 0;

        //iterate across the 2D array
        while i < string_1.len() {
            j = 0;  
            while j < string_2.len() {
                //convert str indexes to chars
                let b1 = string_1.as_bytes()[i];
                let ch1 = b1 as char;
                let b2 = string_2.as_bytes()[j];
                let ch2 = b2 as char;

                if j != 0 && i != 0 {
                    //if the chars match, assign value of square L(i, j) = 1 + value of the upper left diagonal square
                    //if no match, assign L(i, j) = to either the value of the square above or the square to the left, whichever is greater
                    if ch1 == ch2 {
                        lcs_table[i][j] = 1 + lcs_table[i - 1][j - 1];
                    } else if lcs_table[i - 1][j] > lcs_table[i][j - 1] {
                                lcs_table[i][j] = lcs_table[i - 1][j];
                            } else {
                                lcs_table[i][j] = lcs_table[i][j - 1];
                            }
                } else if ch1 == ch2 {
                    lcs_table[i][j] += 1;
                }

                j += 1;
            }
            i += 1;
        }

        //The last square on the lower right of the table (2D array) will have the highest value
        lcs_table[string_1.len() - 1][string_2.len() - 1]
    }

    pub fn knapsack_no_rep(items: &[Item<i32>], capacity: i32) -> i32 {
        let mut solution_space: Vec<Vec<i32>> = Vec::new();
        let mut row: Vec<i32> = Vec::new();

        //populate solution space
        for _i in 0..items.len() {
            row.push(0);
        }

        for _b in 0..capacity {
            let row_ref = &row;
            let v = row_ref.to_vec();
            solution_space.push(v);
        }


        for i in 0..items.len() {
            for b in 0..capacity {
                let u1: usize = i -1;
                let current_max = solution_space[u1][b as usize]; //current largest value
                
                //if current object can "fit" in b
                if items[i].weight <= b {
                    let u2: usize = (b - items[i].weight) as usize;
                    let new_val = items[i].value + solution_space[u1][u2]; //new value items[i].value + last object at b - object weight 
                    if new_val > current_max {
                        solution_space[i as usize][b as usize] = new_val;
                    } else {
                        solution_space[i as usize][b as usize] = current_max;
                    }
                } else {
                    solution_space[i as usize][b as usize] = current_max;
                }
            }
        }

        solution_space[items.len() as usize][capacity as usize]
    }
}
