pub struct Solution;

impl Solution {
    #[must_use]
    pub fn count_matches(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> i32 {
        let mut count = 0;

        for item in items {
            match rule_key {
                "type" => {
                    if item[0] == rule_value {
                        count += 1;
                    }
                }

                "color" => {
                    if item[1] == rule_value {
                        count += 1;
                    }
                }

                "name" => {
                    if item[2] == rule_value {
                        count += 1;
                    }
                }

                _ => {}
            }
        }
        count
    }
}
