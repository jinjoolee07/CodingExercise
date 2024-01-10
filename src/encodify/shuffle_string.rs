pub struct Solution;

impl Solution {
    #[must_use]
    pub fn restore_string(s: &str, indices: &[usize]) -> String {
        let mut result = vec![' '; s.len()];

        for (i, &index) in s.chars().zip(indices.iter()) {
            result[index] = i;
        }

        result.into_iter().collect()
    }
}
