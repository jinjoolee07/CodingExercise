pub struct Solution;

impl Solution {
    #[must_use]
    pub fn minimum_sum(num: i32) -> i32 {
        Self::get_sorted_digits(num)
    }

    fn get_sorted_digits(mut num: i32) -> i32 {
        let mut digits = [0; 4];

        for i in (0..4).rev() {
            digits[i] = num % 10;
            num /= 10;
        }

        digits.sort_unstable();

        digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
    }
}
