use crate::utils::ListNode;

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut max_twin_sum = i32::MIN;
    let mut first_half = vec![];
    let mut p = head.as_ref();
    let mut q = head.as_ref();
    while !q.is_none() {
        let node = p.unwrap();
        first_half.push(node.val);
        p = node.next.as_ref();
        q = q.unwrap().next.as_ref()
            .unwrap().next.as_ref();
    }
    while !p.is_none() {
        let node = p.unwrap();
        let twin_sum = first_half.pop().unwrap() + node.val;
        if twin_sum > max_twin_sum {
            max_twin_sum = twin_sum;
        }
        p = node.next.as_ref();
    }
    max_twin_sum
}

#[cfg(test)]
mod tests {
    use super::pair_sum;
    use crate::utils::ListNode;

    #[test]
    fn test_pair_sum() {
        let head = ListNode::from(&[5,4,2,1]);
        let actual = pair_sum(head);
        assert_eq!(actual, 6);
    }
}
