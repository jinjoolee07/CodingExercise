pub struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];

        for i in 0..nums.len() {
            let index = nums[i] as usize;
            ans[i] = nums[index]
        }
        ans
    }
}
