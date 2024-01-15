pub struct Solution;

impl Solution {
    #[must_use]
    pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
        let mut ans = nums.to_vec();
        ans.extend_from_slice(nums);
        ans
    }
}
