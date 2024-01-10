// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    #[must_use]
    pub fn get_decimal_value(vec: &[i32]) -> i32 {
        let mut result = 0;
        let mut current = None;

        for &val in vec {
            let _node = ListNode::new(val); // Added underscore to indicate intentional unused variable
            current = Some(Box::new(ListNode { val, next: current }));
        }

        let mut current = &current;
        while let Some(node) = current {
            result = (result << 1) + node.val;
            current = &node.next;
        }

        result
    }
}
