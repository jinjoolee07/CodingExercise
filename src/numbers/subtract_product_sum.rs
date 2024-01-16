pub struct Solution;

impl Solution {
    #[must_use]
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (product, sum) = n
            .to_string()
            .chars()
            .map(|c| i32::try_from(c.to_digit(10).unwrap_or(0)).unwrap_or(0))
            .fold((1, 0), |(prod, s), digit| (prod * digit, s + digit));

        product - sum
    }
}
