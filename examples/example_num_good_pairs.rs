use coding_exercise::numbers::num_good_pairs::Solution;

fn main() {
    let input = [1, 2, 3, 1, 1, 3];
    let result = Solution::num_identical_pairs(input);
    println!("{:?}", result);
}
