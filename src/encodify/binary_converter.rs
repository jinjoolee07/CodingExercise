// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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

    // Function to create a linked list from a vector
    #[must_use]
    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &val in vec {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut()?.next;
        }

        head
    }
}

pub struct Solution;

impl Solution {
    #[must_use]
    pub fn get_decimal_value(head: &Option<Box<ListNode>>) -> i32 {
        let mut result = 0;

        let mut current = head;
        while let Some(node) = current {
            result = (result << 1) + node.val;
            current = &node.next;
        }

        result
    }
}
