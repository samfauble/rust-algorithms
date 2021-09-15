pub mod dynamic;
use dynamic::dynamic_algos;
use dynamic::dynamic_algos::Item;

fn main() {
    //lcs
    let s1 = "ldkendjcheeudcxj";
    let s2 = "ldkejfmcjendheedjswkxjasw";
    let str1: String = String::from(s1);
    let str2: String = String::from(s2);
    let lcs_answer = dynamic_algos::lcs(str1, str2);
    println!("{}", lcs_answer);

    //lis
    let mut arr = Vec::new();
    let vals = [32, 29, 1, 10, 3, 4, 2, 20, 21, 19, 5, 22, 14];
    for i in vals {
        arr.push(i);
    }
    let lis_answer = dynamic_algos::lis(arr);
    println!("{}", lis_answer);

    //knapsack no rep
    let item1 = Item::new(3, 1);
    let item2 = Item::new(6, 4);
    let item3 = Item::new(7, 3);
    let item4 = Item::new(10, 6);
    let item5 = Item::new(4, 1);
    let item_arr = [item1, item2, item3, item4, item5];
    let no_rep_answer = dynamic_algos::knapsack_no_rep(&item_arr, 8);
    println!("{}", no_rep_answer);
}