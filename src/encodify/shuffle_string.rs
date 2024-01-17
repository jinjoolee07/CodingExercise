pub struct Solution;

impl Solution {
    #[must_use]
    pub fn restore_string(s: &str, indices: &[usize]) -> String {
        indices
            .iter()
            .copied()
            .zip(s.chars())
            .fold(vec![' '; s.len()], |mut result, (index, char)| {
                result[index] = char;
                result
            })
            .into_iter()
            .collect()
    }
}
