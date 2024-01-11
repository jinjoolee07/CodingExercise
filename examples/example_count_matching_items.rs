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

    let rule_key = "color";
    let rule_value = "silver";

    let result = Solution::count_matches(&items, rule_key, rule_value);
    println!("{:?}", result);
}
