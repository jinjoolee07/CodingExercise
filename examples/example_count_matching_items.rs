use coding_exercise::string::count_matching_items::Solution;

fn main() {
    let items = vec![
        vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
        vec![
            "computer".to_string(),
            "silver".to_string(),
            "lenovo".to_string(),
        ],
        vec![
            "phone".to_string(),
            "gold".to_string(),
            "iphone".to_string(),
        ],
    ];

    let rule_key = "color".to_string();
    let rule_value = "silver".to_string();

    let result = Solution::count_matches(&items, &rule_key, &rule_value);
    println!("Output: {}", result);
}
