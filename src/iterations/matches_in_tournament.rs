pub struct Solution;

impl Solution {
    #[must_use]
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut matches_played = 0;

        while n > 1 {
            matches_played += n / 2;
            n = (n + 1) / 2;
        }

        matches_played
    }
}
