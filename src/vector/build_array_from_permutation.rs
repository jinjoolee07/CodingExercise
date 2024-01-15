pub struct Solution;

impl Solution {
    #[must_use]
    pub fn build_array(nums: &[usize]) -> Vec<usize> {
        nums.iter().map(|&x| nums[x]).collect()
    }
}
