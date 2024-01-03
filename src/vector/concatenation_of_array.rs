pub struct Solution;

impl Solution {
    #[must_use]
    #[allow(clippy::must_use_candidate)]
    pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
        let mut result = Vec::new();

        for &num in nums {
            result.push(num);
        }

        for &num in nums {
            result.push(num);
        }

        result
    }
}
