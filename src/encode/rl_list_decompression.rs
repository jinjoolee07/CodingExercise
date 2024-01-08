pub struct Solution;

impl Solution {
    #[must_use]
    pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
        nums.chunks(2)
            .flat_map(|chunk| {
                let Ok(count) = chunk[0].try_into() else {
                    eprintln!("Error: Negative count encountered.");
                    return Vec::new();
                };

                let value = chunk[1];
                vec![value; count]
            })
            .collect()
    }
}
