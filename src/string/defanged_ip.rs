pub struct Solution;

impl Solution {
    #[must_use]
    pub fn defang_i_paddr(address: &str) -> String {
        address.replace('.', "[.]")
    }
}
