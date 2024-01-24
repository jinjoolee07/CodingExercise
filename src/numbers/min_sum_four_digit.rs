pub struct Solution;

impl Solution {
    /// # Errors
    /// Returns an error if the input is not a four-digit number or if there are parsing errors.
    /// # Panics
    /// This function may panic if there is an error during parsing and the `expect` method is called.
    pub fn minimum_sum(num: i32) -> Result<i32, &'static str> {
        let num_str = num.to_string();
        if num_str.len() != 4 {
            return Err("Input must be a four-digit number");
        }

        let mut digits: Vec<_> = num_str.chars().collect();
        digits.sort_unstable();

        let (new1, new2): (String, String) = digits.chunks(2).try_fold(
            (String::new(), String::new()),
            |(mut a1, mut a2), chunk| {
                if let Some(&c) = chunk.first() {
                    a1.push(c);
                } else {
                    return Err("Out of bounds");
                }

                if let Some(&c) = chunk.get(1) {
                    a2.push(c);
                } else {
                    return Err("Out of bounds");
                }

                Ok((a1, a2))
            },
        )?;

        let result_one = new1
            .parse::<i32>()
            .expect("Parsed result_first should be a valid integer");
        let result_two = new2
            .parse::<i32>()
            .expect("Parsed result_second should be a valid integer");

        Ok(result_one + result_two)
    }
}
