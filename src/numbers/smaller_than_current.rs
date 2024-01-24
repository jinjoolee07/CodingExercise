pub struct Solution;

impl Solution {
    /// # Panics
    /// Panics if attempting to access an index out of bounds.
    #[must_use]
    pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
        (0..nums.len())
            .map(|i| {
                nums.iter()
                    .filter(|&&num| num < *nums.get(i).expect("Indexing out of bounds"))
                    .count()
            })
            .collect()
    }
}
