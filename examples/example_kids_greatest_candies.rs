use coding_exercise::numbers::kids_greatest_candies::Solution;

fn main() {
    let input = [2, 3, 5, 1, 3];
    let n = 3;
    let result = Solution::kids_with_candies(&input, n);

    println!("{:?}", result);
}
