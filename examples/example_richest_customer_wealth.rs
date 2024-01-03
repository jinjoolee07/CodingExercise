use coding_exercise::vector::richest_customer_wealth::Solution;

fn main() {
    let input = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let result = Solution::maximum_wealth(input);
    println!("{:?}", result)
}
