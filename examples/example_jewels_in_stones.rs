use coding_exercise::string::jewels_in_stones::Solution;

fn main() {
    let jewels = "aA";
    let stones = "aAAbbbb";
    let result = Solution::num_jewels_in_stones(jewels, stones);

    println!("{:?}", result);
}
