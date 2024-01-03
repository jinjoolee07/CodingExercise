pub struct Solution;

#[allow(clippy::needless_pass_by_value)]
impl Solution {
    #[must_use]
    #[allow(clippy::must_use_candidate)]
    pub fn running_sum(nums: &[i32]) -> Vec<i32> {
        let mut running_sum = vec![0; nums.len()];

        for i in 0..nums.len() {
            running_sum[i] = if i == 0 {
                nums[i]
            } else {
                running_sum[i - 1] + nums[i]
            };
        }
        running_sum
    }
}
