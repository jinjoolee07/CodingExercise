use coding_exercise::iterations::target_array_order::Solution;

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    let result = Solution::create_target_array(&nums, &index);

    println!("{:?}", result);
}
