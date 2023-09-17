use std::collections::BinaryHeap;

pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let candidates = candidates as usize;
    if 2 * candidates > costs.len() {
        let mut q = BinaryHeap::from(costs);
        let k = k.min(q.len() as i32);
        let mut res = 0i64;
        for _ in 0..k {
            res += q.pop().unwrap() as i64;
        }
        res
    } else {
        let mut q = BinaryHeap::with_capacity(2 * candidates);
        for i in 0..candidates {
            q.push((costs[i], true));
            q.push((costs[costs.len() - i - 1], false));
        }
        let mut res = 0i64;
        let mut left = candidates;
        let mut right = costs.len() - candidates;
        for _ in 0..k {
            let w = q.pop().unwrap();
            res += w.0 as i64;
            if w.1 {
                q.push((costs[left], true));
                left += 1
            } else {
                right -= 1;
                if right >= left {
                    q.push((costs[right], false));
                }
            }
            if left > right {
                break;
            }
        }
        res
    }
}
