use coding_exercise::string::max_words_sentence::Solution;

fn main() {
    let sentences = vec![
        "alice and bob love leetcode",
        "i think so too",
        "this is great thanks very much",
    ];
    let result = Solution::most_words_found(sentences);
    println!("{:?}", result)
}
