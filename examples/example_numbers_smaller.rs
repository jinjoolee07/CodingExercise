use coding_exercise::numbers::smaller_than_current::Solution;

fn main() {
    let input = vec![8, 1, 2, 2, 3];
    let result = Solution::smaller_numbers_than_current(&input);
    println!("{:?}", result);
}
