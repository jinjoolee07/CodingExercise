pub struct Solution;

impl Solution {
    /// # Panics
    /// Panics if attempting to access an index out of bounds.
    #[must_use]
    pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
        nums.iter()
            .map(|&num| nums.iter().filter(|&&n| n < num).count())
            .collect()
    }
}
