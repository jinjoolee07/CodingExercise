pub struct Solution;

impl Solution {
    #[must_use]
    pub fn shuffle(nums: &[i32]) -> Vec<i32> {
        let mut result = Vec::new();
        let n = nums.len() / 2;

        for i in 0..n {
            result.push(nums[i]);
            result.push(nums[i + n]);
        }
        result
    }
}
