pub struct Solution;

impl Solution {
    #[must_use]
    pub fn maximum_wealth(accounts: &[Vec<i32>]) -> Option<i32> {
        accounts
            .iter()
            .map(|customer_accounts| customer_accounts.iter().sum())
            .max()
    }
}
