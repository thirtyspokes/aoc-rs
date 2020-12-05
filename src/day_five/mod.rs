use std::fs;

pub fn solve_day_five() {
    let input = fs::read_to_string("inputs/day-five.txt").unwrap();
    let max = input.lines().map(|line| get_seat_id(line)).max().unwrap();
    println!("Part one: highest seat ID: {}", max);

    let mut all_seats: Vec<i64> = input.lines().map(|line| get_seat_id(line)).collect();
    all_seats.sort_unstable();
    for (idx, seat) in all_seats.iter().enumerate() {
        if (idx < all_seats.len() - 1) && (all_seats[idx + 1] != seat + 1) {
            println!("Part two: the missing seat is {}", seat + 1);
        }
    }
}

fn get_seat_id(boarding_pass: &str) -> i64 {
    let rows = &boarding_pass[0..7];
    let columns = &boarding_pass[7..];

    let row = binary_partition(128, rows, 'F');
    let column = binary_partition(8, columns, 'L');

    (row * 8) + column
}

fn binary_partition(length: i64, input: &str, front_char: char) -> i64 {
    let mut high = length;
    let mut low = 0;

    for c in input.chars() {
        let new_length = (high - low) / 2;

        if c == front_char {
            high = low + new_length;
        } else {
            low = high - new_length;
        }
    }

    low
}
