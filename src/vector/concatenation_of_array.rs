pub struct Solution;

impl Solution {
    #[must_use]
    pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
        nums.iter().copied().chain(nums.iter().copied()).collect()
    }
}
