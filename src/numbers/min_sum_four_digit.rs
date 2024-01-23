pub struct Solution;

impl Solution {
    /// # Errors
    /// Returns an error if the input is not a four-digit number or if there are parsing errors.
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
                a1.push(chunk[0]);
                // Return an error if None is encountered during chunk processing
                let c = chunk.get(1).copied().ok_or("Out of bounds")?;
                a2.push(c);
                Ok((a1, a2))
            },
        )?;

        let result_one = new1
            .parse::<i32>()
            .map_err(|_| "Error parsing result_first")?;
        let result_two = new2
            .parse::<i32>()
            .map_err(|_| "Error parsing result_second")?;

        Ok(result_one + result_two)
    }
}
