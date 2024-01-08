use coding_exercise::encode::message_decoding::Solution;

fn main() {
    let key = "the quick brown fox jumps over the lazy dog".to_string();
    let message = "vkbs bs t suepuv".to_string();

    let result = Solution::decode_message(&key, &message);
    println!("{}", result); // Output: "this is a secret"
}
