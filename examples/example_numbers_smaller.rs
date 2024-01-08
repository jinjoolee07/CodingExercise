use coding_exercise::numbers::smaller_than_current::Solution;

fn main() {
    let input = [6, 5, 4, 8];
    let result = Solution::smaller_than_current(&input);
    println!("{:?}", result);
}
