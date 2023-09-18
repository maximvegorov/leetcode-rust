use std::collections::HashMap;

pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    let mut result = 0;
    let mut dp = Vec::with_capacity(arr.len());
    let mut last_index = HashMap::with_capacity(arr.len());
    for (i, &ni) in arr.iter().enumerate() {
        let l = match last_index.get(&(ni - difference)) {
            Some(&j) => dp[j] + 1,
            None => 1,
        };
        dp.push(l);
        if l > result {
            result = l;
        }
        last_index.insert(ni, i);
    }
    result
}
