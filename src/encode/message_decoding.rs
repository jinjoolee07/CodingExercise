pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let substitution_table = create_substitution_table(&key);

        let decoded_message: String = message
            .chars()
            .map(|ch| substitution_table.get(&ch).copied().unwrap_or_else(|| ch))
            .collect();

        decoded_message
    }
}
