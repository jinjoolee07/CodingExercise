pub struct Solution;

impl Solution {
    #[must_use]
    pub fn running_sum(nums: &[i32]) -> Vec<i32> {
        nums.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }
}
