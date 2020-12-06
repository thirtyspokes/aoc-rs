use std::fs;

pub fn solve_day_five() {
    let input = fs::read_to_string("inputs/day-five.txt").unwrap();
    let mut all_seat_ids: Vec<i64> = input.lines().map(|line| get_seat_id(line)).collect();

    // We have a function that can calculate the seat ID for a given
    // boarding pass, so for part one we just need to map over all the passes
    // and find the highest number.
    let max = all_seat_ids.iter().max().unwrap();
    println!("Part one: highest seat ID: {}", max);

    // Part two involves finding the seat that isn't in the list of passes,
    // given that the seat numbers are sequentially assigned and the seat ID
    // above and below ours exists in the list.
    //
    // To do that, we'll sort all the seat IDs and then search for an instance
    // where one is missing.
    all_seat_ids.sort_unstable();
    for (idx, seat) in all_seat_ids.iter().enumerate() {
        if (idx < all_seat_ids.len() - 1) && (all_seat_ids[idx + 1] != seat + 1) {
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

// To perform a binary partition, we're just cutting the sequence
// in half - the characters we iterate over determines whether we're taking
// the top or bottom half.
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
