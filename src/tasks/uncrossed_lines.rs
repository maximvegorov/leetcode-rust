pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    return Solver::new(nums1, nums2)
        .solve(0, 0);
}

struct Solver {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    dp: Vec<Vec<i32>>,
}

impl Solver {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut dp = Vec::with_capacity(nums1.len());
        for i in 1..=nums1.len() {
            dp.push(vec![-1; nums2.len()])
        }
        Self { nums1, nums2, dp }
    }

    fn solve(&mut self, i: usize, j: usize) -> i32 {
        if i >= self.nums1.len() || j >= self.nums2.len() {
            return 0;
        }
        let mut result = self.dp[i][j];
        if result != -1 {
            return result;
        }
        for k in i..self.nums1.len() {
            let v = self.nums1[k];
            for l in j..self.nums2.len() {
                if self.nums2[l] == v {
                    result = result.max(self.solve(k + 1, l + 1) + 1);
                    break;
                }
            }
        }
        result = result.max(0);
        self.dp[i][j] = result;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::uncrossed_lines::max_uncrossed_lines;

    #[test]
    fn test_max_uncrossed_lines() {
        let actual = max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]);
        assert_eq!(actual, 3);
    }
}
