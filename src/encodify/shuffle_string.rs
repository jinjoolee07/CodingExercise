pub struct Solution;

impl Solution {
    #[must_use]
    pub fn restore_string(s: &str, indices: &[usize]) -> String {
        let mut result = vec![' '; s.len()];

        for (index, char) in indices.iter().zip(s.chars()) {
            result[*index] = char;
        }

        result.into_iter().collect()
    }
}
