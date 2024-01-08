pub struct Solution;

// coding_exercise/src/encode/ll_to_integer.rs

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

impl Solution {
    #[must_use]
    pub fn get_decimal_value(head: &Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut current = head;

        while let Some(node) = current {
            result = (result << 1) | node.val;
            current = &node.next;
        }

        result
    }
}
