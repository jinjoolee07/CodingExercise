pub struct Solution;

impl Solution {
    /// # Panics
    /// Panics if the conversion from usize to i32 fails.
    #[must_use]
    pub fn count_matches(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> i32 {
        i32::try_from(
            items
                .iter()
                .filter(|item| match rule_key {
                    "type" => item.first() == Some(&rule_value.to_string()),
                    "color" => item.get(1) == Some(&rule_value.to_string()),
                    "name" => item.get(2) == Some(&rule_value.to_string()),
                    _ => false,
                })
                .count(),
        )
        .expect("usize count of matches should be less than or equal to i32 max; conversion is expected to be safe")
    }
}
