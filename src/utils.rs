use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn from(vals: &[i32]) -> Option<Box<ListNode>> {
        if vals.is_empty() {
            return None;
        }

        let mut head = ListNode::new(0);
        let mut p = &mut head.next;
        for &val in vals {
            *p = Some(Box::new(ListNode::new(val)));
            p = &mut p.as_mut().unwrap().next;
        }
        head.next
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
