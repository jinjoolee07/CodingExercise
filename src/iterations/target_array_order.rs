pub struct Solution;

impl Solution {
    #[must_use]
    pub fn create_target_array(nums: &[i32], index: &[usize]) -> Vec<i32> {
        let mut target = Vec::new();

        for (&num, &idx) in nums.iter().zip(index.iter()) {
            target.insert(idx, num);
        }

        target
    }
}
