pub fn find_number_of_lis(mut nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut dp: Vec<(i32, i32)> = Vec::with_capacity(nums.len());
    for (i, &ni) in nums.iter().enumerate() {
        let mut l = 0;
        let mut cnt = 1;
        for (j, &nj) in nums[..i].iter().enumerate() {
            if nj < ni {
                let lj = dp[j].0;
                if l <= lj {
                    if l == lj {
                        cnt += dp[j].1;
                    } else {
                        cnt = dp[j].1;
                        l = lj;
                    }
                }
            }
        }
        l += 1;
        dp.push((l, cnt));
        if l > result {
            result = l;
        }
    }
    dp.into_iter().filter(|p| (*p).0 == result).map(|p| p.1).sum()
}

#[cfg(test)]
mod tests {
    use crate::tasks::find_number_of_lis::find_number_of_lis;

    #[test]
    fn test_find_number_of_lis() {
        let actual = find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]);
        assert_eq!(actual, 3);
    }
}
