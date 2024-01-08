pub struct Solution;

impl Solution {
    #[must_use]
    pub fn restore_string(s: &str, indices: &[i32]) -> String {
        let mut result = vec![' '; s.len()];

        for (ch, &index) in s.chars().zip(indices.iter()) {
            if let Ok(index_usize) = usize::try_from(index) {
                result[index_usize] = ch;
            } else {
                eprintln!("Error: Negative index encountered.");
                return String::new();
            }
        }

        result.into_iter().collect()
    }
}
