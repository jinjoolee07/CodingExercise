pub struct Solution;

impl Solution {
    #[must_use]
    pub fn minimum_sum(num: i32) -> usize {
        let mut nums: Vec<_> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as usize)
            .collect();

        nums.sort_unstable();

        nums[0] + nums[1] + nums[2] + nums[3]
    }
}
