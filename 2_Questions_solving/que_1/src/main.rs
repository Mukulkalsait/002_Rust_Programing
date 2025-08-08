mod logic;
use logic::find_min_max_sums;

fn main() {
    let arr = vec![1, 2, 5, 3, 4, 6, 7, 2, 3, 34, 54, 22, 203];
    let k = 4;
    let n = arr.len();

    let res = find_min_max_sums(n, arr, k);
    println!("{:?}", res);

    let arr2 = vec![5, 3, 9, 1, 7];
    let res = find_min_max_sums(5, arr2, 3);
    println!("{:?}", res);
}
