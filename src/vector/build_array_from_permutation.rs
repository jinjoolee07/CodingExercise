pub struct Solution;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]

    pub fn build_array(nums: &[i32]) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];

        for i in 0..nums.len() {
            #[allow(clippy::cast_sign_loss)]
            let index = if nums[i] >= 0 { nums[i] as usize } else { 0 };
            ans[i] = nums[index];
        }
        ans
    }
}
