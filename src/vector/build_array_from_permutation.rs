pub struct Solution;

impl Solution {
    #[must_use]
    pub fn build_array(nums: &[usize]) -> Vec<usize> {
        nums.iter().filter_map(|&x| nums.get(x).copied()).collect()
    }
}
