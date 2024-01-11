use coding_exercise::encodify::runlength_decompress::Solution;

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::decompress_rl_elist(&nums);
    println!("{:?}", result);
}
