pub struct Solution;

impl Solution {
    #[must_use]
    pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
        let max_candies = candies.iter().copied().max().unwrap_or(0);
        candies
            .iter()
            .map(|&kid_candies| kid_candies + extra_candies >= max_candies)
            .collect()
    }
}
