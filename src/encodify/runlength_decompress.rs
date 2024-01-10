pub struct Solution;

impl Solution {
    #[must_use]
    pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
        let mut result = Vec::new();

        for i in (0..nums.len()).step_by(2) {
            let freq = usize::try_from(nums[i]).unwrap_or(1);
            let val = nums[i + 1];

            result.extend(std::iter::repeat(val).take(freq));
        }

        result
    }
}
