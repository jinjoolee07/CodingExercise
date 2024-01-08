pub struct Solution;
use std::collections::HashSet;

impl Solution {
    #[must_use]
    pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
        let jewel_set: HashSet<_> = jewels.chars().collect();

        i32::try_from(
            stones
                .chars()
                .filter(|&stone| jewel_set.contains(&stone))
                .count(),
        )
        .unwrap_or(0)
    }
}
