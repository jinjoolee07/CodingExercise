pub struct Solution;

impl Solution {
    #[must_use]
    pub fn sort_sentence(s: &str) -> String {
        let mut words: Vec<&str> = s.split_whitespace().collect();

        words.sort_by_key(|word| word.chars().last().unwrap_or('O'));

        let reconstructed_sentence: String = words
            .into_iter()
            .map(|word| &word[..word.len() - 1])
            .collect::<Vec<&str>>()
            .join(" ");

        reconstructed_sentence
    }
}
