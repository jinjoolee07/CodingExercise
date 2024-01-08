// examples/example_ll_to_integer.rs

use coding_exercise::encode::ll_to_integer::{ListNode, Solution};

fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));

    let result = Solution::get_decimal_value(&head);
    println!("{:?}", result);
}
