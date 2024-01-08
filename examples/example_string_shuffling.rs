use coding_exercise::encode::string_shuffling::Solution;

fn main() {
    let s = String::from("codeleet");
    let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];
    let result = Solution::restore_string(&s, &indices);
    println!("{}", result); // Output: "leetcode"
}
