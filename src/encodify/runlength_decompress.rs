pub struct Solution;

impl Solution {
    /// # Panics
    #[must_use]
    pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
        nums.chunks(2)
            .flat_map(|chunk| {
                let freq = *chunk
                    .first()
                    .expect("Frequency should be present in a non-empty chunk");
                let val = *chunk
                    .get(1)
                    .expect("Value should be present in a chunk with at least one element");
                vec![
                    val;
                    freq.try_into()
                        .expect("Frequency should be a non-negative integer")
                ]
            })
            .collect()
    }
}
