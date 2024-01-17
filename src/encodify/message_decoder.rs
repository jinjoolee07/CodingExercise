pub struct Solution;
use std::collections::HashMap;

impl Solution {
    #[must_use]
    pub fn decode_message(key: &str, message: &str) -> String {
        let mut map = HashMap::new();
        let mut value = b'a';

        for ch in key.chars() {
            if ch != ' ' && !map.contains_key(&ch) {
                map.insert(ch, value as char);
                value += 1;
            }
        }

        message
            .chars()
            .map(|ch| *map.get(&ch).unwrap_or(&ch))
            .collect()
    }
}
