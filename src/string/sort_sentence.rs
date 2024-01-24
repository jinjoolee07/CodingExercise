pub struct Solution;

impl Solution {
    /// # Panics
    /// Panics if a word is found to be empty (containing no characters).
    #[must_use]
    pub fn sort_sentence(s: &str) -> String {
        let mut words: Vec<&str> = s.split_whitespace().collect();

        words.sort_unstable_by_key(|word| {
            word.chars()
                .last()
                .expect("Each word should have at least one character.")
        });

        words
            .iter()
            .map(|word| {
                word.get(..word.len() - 1)
                    .expect("Unexpected empty word during slicing")
            })
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
