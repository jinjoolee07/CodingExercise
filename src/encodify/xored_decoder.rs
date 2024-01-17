pub struct Solution;

impl Solution {
    #[must_use]
    pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
        let mut arr = vec![first];
        arr.extend(encoded.iter().scan(first, |state, &x| {
            *state ^= x;
            Some(*state)
        }));
        arr
    }
}
