use coding_exercise::vector::shuffle_the_array::Solution;

fn main() {
    let input = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    let result = Solution::shuffle(input, n);
    println!("{:?}", result);
}
