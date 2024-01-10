pub struct Solution;

impl Solution {
    #[must_use]
    pub fn most_words_found(sentences: Vec<&str>) -> usize {
        let mut max_word_count = 0;

        for sentence in sentences {
            let word_count = sentence.split_whitespace().count();
            if word_count > max_word_count {
                max_word_count = word_count;
            }
        }
        max_word_count
    }
}
