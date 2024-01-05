pub struct Solution;

impl Solution {
    #[must_use]
    pub fn maximum_wealth(accounts: &[Vec<i32>]) -> i32 {
        let mut maximum_wealth = i32::MIN;

        for customer_accounts in accounts {
            let customer_wealth: i32 = customer_accounts.iter().sum();

            maximum_wealth = maximum_wealth.max(customer_wealth);
        }
        maximum_wealth
    }
}
