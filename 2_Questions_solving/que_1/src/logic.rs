use std::iter::Enumerate;

pub fn find_min_max_sums(n: usize, arr: Vec<i32>, k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut temp = arr.clone();

    for i in 0..k {
        if temp.is_empty() {
            break;
        }
        if i % 2 == 0 {
            let enumer = temp.iter().enumerate();
            let max_pair = enumer.max_by_key(|(_, x)| *x);
            if let Some((i, &max_val)) = max_pair {
                temp.remove(i);
            }
        } else {
            let enumer = temp.iter().enumerate();
            let min_pair = enumer.min_by_key(|(_, x)| *x);

            if let Some((i, &max_val)) = min_pair {
                temp.remove(i);
            }
        }

        let sum: i32 = temp.iter().sum();
        result.push(sum);
    }
    result
}
