use coding_exercise::vector::build_array_from_permutation::Solution;

fn main() {
    let input = vec![0, 2, 1, 5, 3, 4];
    let result = Solution::build_array(&input);
    println!("{:?}", result);
}
