use std::cmp::Ordering;

pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_by_key(|a| a[1]);
    let mut result = 0;
    let mut dp = Vec::with_capacity(pairs.len());
    for (i, p) in pairs.iter().enumerate() {
        let mut l = 0;
        for (j, t) in pairs[..i].iter().enumerate() {
            if t[1] < p[0] {
                l = l.max(dp[j])
            }
        }
        l += 1;
        dp.push(l);
        if l > result {
            result = l;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::tasks::find_longest_chain::find_longest_chain;

    #[test]
    fn test_find_longest_chain() {
        let actual = find_longest_chain(vec![
            vec![-10,-8], vec![8,9], vec![-5,0], vec![6,10],
            vec![-6,-4], vec![1,7], vec![9,10], vec![-4,7],
        ]);
        assert_eq!(actual, 4);
    }
}
