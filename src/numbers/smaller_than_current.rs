pub struct Solution;
use std::collections::HashMap;

impl Solution {
    #[must_use]
    pub fn smaller_than_current(nums: &[i32]) -> Vec<i32> {
        let mut num_count = HashMap::new();
        let mut result = Vec::new();

        for &num in nums {
            let count = num_count.entry(num).or_insert(0);
            result.push(*count);
            *count += 1;
        }

        for i in 0..result.len() {
            let num = nums[i];
            let smaller_count = (0..num).map(|x| num_count.get(&x).unwrap_or(&0)).sum();
            result[i] = smaller_count;
        }

        result
    }
}
