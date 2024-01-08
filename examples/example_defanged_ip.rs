use coding_exercise::string::defanged_ip::Solution;

fn main() {
    let input = "1.1.1.1";
    let result = Solution::defang_i_paddr(input);
    println!("{:?}", result);
}
