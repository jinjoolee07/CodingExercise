pub struct Solution;

impl Solution {
    #[must_use]
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits: Vec<_> = num.to_string().chars().collect();
        digits.sort_unstable();

        let (new1, new2) =
            digits
                .chunks(2)
                .fold((String::new(), String::new()), |(mut a1, mut a2), chunk| {
                    a1.push(chunk[0]);
                    a2.push(chunk[1]);
                    (a1, a2)
                });

        let new1 = new1.parse::<i32>().unwrap_or(0);
        let new2 = new2.parse::<i32>().unwrap_or(0);

        new1 + new2
    }
}
