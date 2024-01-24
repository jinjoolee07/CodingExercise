pub struct Solution;

impl Solution {
    #[must_use]
    pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
        let mut result = Vec::new();

        for i in 0..nums.len() {
            let count_smaller = nums.iter().filter(|&&num| num < nums[i]).count();
            result.push(count_smaller);
        }

        result
    }
}
