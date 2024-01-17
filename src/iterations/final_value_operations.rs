pub struct Solution;

impl Solution {
    #[must_use]
    pub fn final_value_after_operations(operations: Vec<&str>) -> i32 {
        let mut x = 0;

        for operation in operations {
            match operation {
                "++X" | "X++" => x += 1,
                "--X" | "X--" => x -= 1,
                _ => {}
            }
        }
        x
    }
}
