pub struct Solution;

impl Solution {
    #[must_use]
    pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
        (0..nums.len())
            .map(|i| nums.iter().filter(|&&num| num < nums[i]).count())
            .collect()
    }
}
