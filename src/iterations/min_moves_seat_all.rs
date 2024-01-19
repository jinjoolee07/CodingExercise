pub struct Solution;

impl Solution {
    #[must_use]
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();

        students
            .iter()
            .zip(seats.iter())
            .map(|(&student_pos, &seat_pos)| (student_pos - seat_pos).abs())
            .sum()
    }
}
