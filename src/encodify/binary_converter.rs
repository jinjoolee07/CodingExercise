pub struct Solution;

impl Solution {
    #[must_use]
    pub fn get_decimal_value(vec: &[i32]) -> i32 {
        let mut result = 0;

        for &val in vec {
            result = (result << 1) | val;
        }

        result
    }
}
