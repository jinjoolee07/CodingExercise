use coding_exercise::encodify::xored_decoder::Solution;

fn main() {
    let encoded = vec![1, 2, 3];
    let first = 1;
    let result = Solution::decode(&encoded, first);

    println!("{:?}", result)
}
