pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    return Solver::new(s1_bytes, s2_bytes)
        .solve(s1_bytes.len(), s2_bytes.len());
}

struct Solver<'a> {
    s1: &'a [u8],
    s2: &'a [u8],
    dp: Vec<Vec<i32>>,
}

impl<'a> Solver<'a> {
    fn new(s1: &'a [u8], s2: &'a [u8]) -> Self {
        let mut dp = Vec::with_capacity(s1.len() + 1);
        for i in 1..=s1.len() + 1 {
            dp.push(vec![-1; s2.len() + 1])
        }
        Self { s1, s2, dp }
    }

    fn solve(&mut self, i: usize, j: usize) -> i32 {
        let mut result = self.dp[i][j];
        if result != -1 {
            return result;
        }

        if i == 0 {
            result = self.s2[..j].iter()
                .map(|c| *c as i32)
                .sum();
        } else if j == 0 {
            result = self.s1[..i].iter()
                .map(|c| *c as i32)
                .sum();
        } else {
            let si = self.s1[i - 1] as i32;
            let sj = self.s2[j - 1] as i32;
            if si == sj {
                result = self.solve(i - 1, j - 1);
            } else {
                result = self.solve(i - 1, j) + si;
                result = result.min(self.solve(i, j - 1) + sj);
                result = result.min(self.solve(i - 1, j - 1) + si + sj)
            }
        }

        self.dp[i][j] = result;

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::minimum_delete_sum::minimum_delete_sum;

    #[test]
    fn test_max_uncrossed_lines() {
        let actual = minimum_delete_sum("sea".to_string(), "eat".to_string());
        assert_eq!(actual, 231);
    }
}
