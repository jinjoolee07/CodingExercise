use coding_exercise::iterations::min_moves_seat_all::Solution;

fn main() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    let result = Solution::min_moves_to_seat(seats, students);
    println!("{:?}", result);
}
