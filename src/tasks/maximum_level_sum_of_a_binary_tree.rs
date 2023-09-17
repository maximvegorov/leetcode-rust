use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::utils::TreeNode;

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut level = 1;
    let mut max_level = 1;
    let mut max_sum = root.as_ref().unwrap().borrow().val;
    let mut q = VecDeque::new();
    q.push_back(root);
    q.push_back(None);
    let mut level_sum = 0;
    while !q.is_empty() {
        match q.pop_front().unwrap() {
            None => {
                if level_sum > max_sum {
                    max_level = level;
                    max_sum = level_sum;
                }
                max_level += 1;
                level_sum = 0;
                if !q.is_empty() {
                    q.push_back(None);
                }
            }
            Some(node) => {
                let node_ref = node.borrow();
                level_sum += node_ref.val;
                if !node_ref.left.is_none() {
                    q.push_back(node_ref.left.clone())
                }
                if !node_ref.right.is_none() {
                    q.push_back(node_ref.right.clone())
                }
            }
        }
    }
    max_level
}
