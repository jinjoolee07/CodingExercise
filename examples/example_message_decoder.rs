use coding_exercise::encodify::message_decoder::Solution;

fn main() {
    let key = "the quick brown fox jumps over the lazy dog";
    let message = "vkbs bs t suepuv";
    let result = Solution::decode_message(key, message);
    println!("{:?}", result);
}
