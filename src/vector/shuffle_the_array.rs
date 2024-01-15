pub struct Solution;

impl Solution {
    #[must_use]
    pub fn shuffle(nums: &[i32]) -> Vec<i32> {
        (0..nums.len() / 2)
            .flat_map(|i| vec![nums[i], nums[i + nums.len() / 2]])
            .collect()
    }
}
