pub struct Solution;

impl Solution {
    #[must_use]
    pub fn number_of_steps(num: i32) -> u32 {
        num.count_ones() + (31 - num.leading_zeros())
    }
}
