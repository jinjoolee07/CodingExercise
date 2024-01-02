impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        //initialize max wealth to small value
        let mut maximum_wealth = i32::MIN;

        //iteratate through customer accounts
        for customer_accounts in &accounts {
            //iterate through customer account and add all values
            let customer_wealth: i32 = customer_accounts.iter().sum();

            //compare current wealth value to customer wealth and return the bigger value
            maximum_wealth = maximum_wealth.max(customer_wealth);
        }

        //return max wealth
        maximum_wealth
    }
}
