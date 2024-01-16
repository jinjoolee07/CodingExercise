pub struct Solution;

impl Solution {
    #[must_use]
    pub fn count_matches(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> i32 {
        i32::try_from(
            items
                .iter()
                .filter(|item| match rule_key {
                    "type" => item[0] == rule_value,
                    "color" => item[1] == rule_value,
                    "name" => item[2] == rule_value,
                    _ => false,
                })
                .count(),
        )
        .unwrap_or_default()
    }
}
