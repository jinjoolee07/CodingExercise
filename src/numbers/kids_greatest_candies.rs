pub struct Solution;

impl Solution {
    #[must_use]
    pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
        let mut max = 0;
        let mut ans = Vec::new();

        for &candy in candies {
            max = std::cmp::max(max, candy);
            ans.push(candy + extra_candies >= max);
        }
        ans
    }
}
