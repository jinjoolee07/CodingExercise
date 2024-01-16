pub struct Solution;
use std::collections::HashMap;

impl Solution {
    #[must_use]
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut num_count = HashMap::new();
        let mut count = 0;

        for num in nums {
            count += *num_count
                .entry(num)
                .and_modify(|value| *value += 1)
                .or_insert(0);
        }

        count
    }
}
