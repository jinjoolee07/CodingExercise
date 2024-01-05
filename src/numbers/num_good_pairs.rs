pub struct Solution;
use std::collections::HashMap;

impl Solution {
    #[must_use]
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut num_count = HashMap::new();

        for num in nums {
            let value = num_count.entry(num).or_insert(0);
            count += *value;
            *value += 1;
        }

        count
    }
}
