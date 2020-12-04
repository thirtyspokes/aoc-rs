use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/day-two.txt").expect("Oh no");
    let mut total = 0;
    let mut total_two = 0;
    for line in contents.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let range: Vec<_> = parts[0].split("-").collect();
        let min = range[0].parse::<usize>().unwrap();
        let max = range[1].parse::<usize>().unwrap();
        let target = parts[1].chars().next().unwrap();
        let results = parts[2];
        let mut count = 0;

        if (results.chars().nth(min - 1).unwrap() == target)
            ^ (results.chars().nth(max - 1).unwrap() == target)
        {
            println!(
                "Valid: {} appears in positions {} and {} in {}",
                target, min, max, results
            );
            total_two += 1;
        }

        for c in results.chars() {
            if c == target {
                count += 1;
            }
        }

        if count >= min && count <= max {
            println!("Valid: {} appears {} times in {}", target, count, line);
            total += 1;
        }
    }

    println!("{} matches", total);
    println!("{} part two matches", total_two);
}
