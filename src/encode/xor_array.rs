pub struct Solution;

impl Solution {
    #[must_use]
    pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
        let mut result = vec![first];

        for &num in encoded {
            let next_element = result.last().unwrap_or(&0) ^ num;
            result.push(next_element);
        }

        result
    }
}
