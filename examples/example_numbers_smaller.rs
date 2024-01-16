use coding_exercise::numbers::smaller_than_current::Solution;

fn main() {
    let input = vec![6, 5, 4, 8];
    let result = Solution::smaller_numbers_than_current(&input);
    println!("{:?}", result);
}
