pub struct Solution;

impl Solution {
    #[must_use]
    pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
        nums.chunks(2)
            .flat_map(|chunk| {
                let freq = usize::try_from(chunk[0]).unwrap_or(0);
                let val = chunk[1];
                vec![val; freq]
            })
            .collect()
    }
}
