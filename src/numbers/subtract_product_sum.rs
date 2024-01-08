pub struct Solution;

impl Solution {
    #[must_use]
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut num = n;
        let mut product = 1;
        let mut add = 0;

        while num > 0 {
            let digit = num % 10;
            product *= digit;
            add += digit;
            num /= 10;
        }

        product - add
    }
}
