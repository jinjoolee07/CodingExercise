use coding_exercise::encode::rl_list_decompression::Solution;

fn main() {
    let input = [1, 2, 3, 4];
    let result = Solution::decompress_rl_elist(&input.to_vec());
    println!("{:?}", result);
}
