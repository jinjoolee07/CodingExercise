use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[must_use]
    /// # Panics
    /// This function may panic if the binary search result is not within the bounds of the sorted array.
    pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
        let mut count_map = HashMap::new();
        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort_unstable();

        for (i, &num) in sorted_nums.iter().enumerate() {
            count_map.entry(num).or_insert(i);
        }

        nums.iter()
            .map(|&num| {
                i32::try_from(*count_map.get(&num).unwrap())
                    .expect("Count retrieval from HashMap should always return Some value")
            })
            .collect()
    }
}
