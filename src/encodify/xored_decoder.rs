pub struct Solution;

impl Solution {
    #[must_use]
    pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
        let mut arr = Vec::with_capacity(encoded.len() + 1);
        arr.push(first);

        for &encode in encoded {
            if let Some(&last) = arr.last() {
                arr.push(last ^ encode);
            } else {
                return Vec::new();
            }
        }

        arr
    }
}
