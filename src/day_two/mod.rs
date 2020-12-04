use std::fs;

pub fn solve_day_two() {
    let contents = fs::read_to_string("inputs/day-two.txt").expect("Oh no");

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for line in contents.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let range: Vec<_> = parts[0].split('-').collect();

        let min = range[0].parse::<usize>().unwrap();
        let max = range[1].parse::<usize>().unwrap();

        let target = parts[1].chars().next().unwrap();
        let search_string = parts[2];

        let mut count = 0;

        // For part two, the min and max are target indicies, and
        // we count the password as valid if the target character
        // exists at only one of them.
        if (search_string.chars().nth(min - 1).unwrap() == target)
            ^ (search_string.chars().nth(max - 1).unwrap() == target)
        {
            part_two_total += 1;
        }

        // For part one, if the target character exists between
        // min and max times in the string, it's a valid password.
        for c in search_string.chars() {
            if c == target {
                count += 1;
            }
        }

        if count >= min && count <= max {
            part_one_total += 1;
        }
    }

    println!("Part one: {} matches", part_one_total);
    println!("Part two: {} matches", part_two_total);
}
