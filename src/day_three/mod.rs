use std::fs;

pub fn solve_day_three() {
    let contents = fs::read_to_string("inputs/day-three.txt").expect("Oh no");

    // Part one
    let part_one_trees = solve_path(&contents, 3, 1);
    println!("For part one there are {} trees.", part_one_trees);

    // Part two
    let a = solve_path(&contents, 1, 1);
    let b = solve_path(&contents, 5, 1);
    let c = solve_path(&contents, 7, 1);
    let d = solve_path(&contents, 1, 2);

    println!(
        "For part two, the final value is {}",
        a * b * c * d * part_one_trees
    );
}

fn solve_path(input: &str, right: usize, down: usize) -> i64 {
    let mut x = 0;
    let mut y = 1;
    let mut trees = 0;

    for line in input.lines() {
        // Handle instances where we want to move down more than
        // one line at a time the same way we handle moving rightward.
        y += 1;
        if y <= (down - 1) {
            continue;
        }
        y = 0;

        let length = line.len() - 1;
        if line.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }

        // The pattern we're traversing repeats itself infinitely
        // until we hit the bottom, so we want to loop the xval around
        // when we've hit the rightward edge.
        x += right;
        if x > length {
            x -= length + 1;
        }
    }

    trees
}
