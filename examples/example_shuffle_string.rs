use coding_exercise::encodify::shuffle_string::Solution;

fn main() {
    let s = "codeleet";
    let indicies = vec![4, 5, 6, 7, 0, 2, 1, 3];
    let result = Solution::restore_string(s, &indicies);
    println!("{:?}", result);
}
