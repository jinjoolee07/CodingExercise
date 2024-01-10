pub struct Solution;
use std::collections::HashSet;

impl Solution {
    #[must_use]
    pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
        let mut jewel_set = HashSet::new();

        for jewel in jewels.chars() {
            jewel_set.insert(jewel);
        }

        let mut count = 0;

        for stone in stones.chars() {
            if jewel_set.contains(&stone) {
                count += 1;
            }
        }

        count
    }
}
