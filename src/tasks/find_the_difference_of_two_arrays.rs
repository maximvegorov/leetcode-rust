use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let s1: HashSet<_> = nums1.into_iter().collect();
    let s2: HashSet<_> = nums2.into_iter().collect();
    let res1: Vec<_> = s1.iter()
        .filter(|n| { !s2.contains(n) })
        .map(|n| { *n })
        .collect();
    let res2: Vec<_> = s2.iter()
        .filter(|n| { !s1.contains(n) })
        .map(|n| { *n })
        .collect();
    Vec::from([res1, res2])
}
