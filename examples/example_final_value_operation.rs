use coding_exercise::iterations::final_value_operations::Solution;

fn main() {
    let input = vec!["--X", "X++", "X++"];
    let result = Solution::final_value_after_operations(input);

    println!("{:?}", result);
}
