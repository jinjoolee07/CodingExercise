pub struct Solution;

impl Solution {
    #[must_use]
    pub fn count_matches(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> i32 {
        i32::try_from(
            items
                .iter()
                .filter(|item| {
                    let (item_type, item_color, item_name) =
                        (item[0].as_str(), item[1].as_str(), item[2].as_str());

                    match rule_key {
                        "type" => item_type == rule_value,
                        "color" => item_color == rule_value,
                        "name" => item_name == rule_value,
                        _ => false,
                    }
                })
                .count(),
        )
        .unwrap_or_default()
    }
}
