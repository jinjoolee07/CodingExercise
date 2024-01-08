use coding_exercise::encode::xor_array::Solution;

fn main() {
    let encoded = [1, 2, 3];
    let first = 1;
    let result = Solution::decode(&encoded.to_vec(), first);
    println!("{:?}", result);
}
