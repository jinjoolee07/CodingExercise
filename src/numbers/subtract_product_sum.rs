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
                        .expect("Conversion from char to digit failed"),
                )
                .expect("Conversion from digit to i32 failed")
            })
            .fold((1, 0), |(prod, s), digit| (prod * digit, s + digit));

        product - sum
    }
}
