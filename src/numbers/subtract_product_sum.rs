pub struct Solution;

impl Solution {
    #[must_use]
    /// # Panics
    /// Panics if the conversion from char to digit or digit to i32 fails.
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (product, sum) = n
            .to_string()
            .chars()
            .map(|c| {
                i32::try_from(
                    c.to_digit(10)
                        .expect("Digit should be present while converting char to digit"),
                )
                .expect("Valid i32 should be obtained while converting digit to i32")
            })
            .fold((1, 0), |(prod, s), digit| (prod * digit, s + digit));

        product - sum
    }
}
