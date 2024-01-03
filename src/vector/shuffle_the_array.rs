pub struct Solution;

impl Solution {
    #[must_use]
    #[allow(clippy::must_use_candidate)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_sign_loss)]
    pub fn shuffle(nums: &[i32], n: i32) -> Vec<i32> {
        let mut result = Vec::new();

        for i in 0..n {
            if i > 0 && i < nums.len() as i32 {
                result.push(nums[i as usize]);
                result.push(nums[(i + n) as usize]);
            }
        }
        result
    }
}
