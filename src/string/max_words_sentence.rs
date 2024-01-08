pub struct Solution;

impl Solution {
    #[must_use]
    pub fn most_words_found(sentences: &[&str]) -> i32 {
        sentences
            .iter()
            .map(|sentence| i32::try_from(sentence.split_whitespace().count()).unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}
