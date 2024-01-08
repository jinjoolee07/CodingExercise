pub struct Solution;

impl Solution {
    #[must_use]
    pub fn build_array(nums: &[usize]) -> Vec<usize> {
        let mut result = Vec::new();
        let n = nums.len();

        for i in 0..n {
            result.push(nums[nums[i]]);
        }

        result
    }
}
