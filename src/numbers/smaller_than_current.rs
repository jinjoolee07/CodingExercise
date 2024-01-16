pub struct Solution;

impl Solution {
    #[must_use]
    pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort_unstable();

        nums.iter()
            .map(|&num| i32::try_from(sorted_nums.binary_search(&num).unwrap_or(0)).unwrap_or(0))
            .collect()
    }
}
