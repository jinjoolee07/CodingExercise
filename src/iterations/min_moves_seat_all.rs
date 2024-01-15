pub struct Solution;

impl Solution {
    #[must_use]
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut move_count = 0;
        let mut seat = seats;
        let mut student = students;

        seat.sort_unstable();
        student.sort_unstable();

        for (s, st) in student.iter().zip(seat.iter()) {
            move_count += (s - st).abs();
        }
        move_count
    }
}
