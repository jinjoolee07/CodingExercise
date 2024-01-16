pub struct Solution;

impl Solution {
    #[must_use]
    pub fn most_words_found(sentences: &[&str]) -> usize {
        sentences
            .iter()
            .map(|sentence| sentence.split_whitespace().count())
            .max()
            .unwrap_or(0)
    }
}
