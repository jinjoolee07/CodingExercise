use coding_exercise::encodify::binary_converter::Solution;

fn main() {
    let input = vec![1, 0, 1];
    let result = Solution::get_decimal_value(&input);
    println!("{:?}", result);
}
