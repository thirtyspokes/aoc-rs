use std::collections::HashSet;
use std::fs;

fn parse_input() -> (HashSet<i64>, Vec<i64>) {
    let contents = fs::read_to_string("inputs/day-one.txt").expect("Oh no");
    let mut values = HashSet::new();
    let mut vector = Vec::new();
    for line in contents.lines() {
        let value = line.parse::<i64>().unwrap();
        values.insert(value);
        vector.push(value);
    }

    return (values, vector);
}

pub fn part_one() -> i64 {
    let (values, _) = parse_input();

    for value in &values {
        let difference = 2020 - *value;
        if values.contains(&difference) {
            return difference * value;
        }
    }

    panic!("No solution was found.")
}

pub fn part_two() -> i64 {
    let (values, vector) = parse_input();

    for i in &vector {
        for j in &vector {
            let partial_sum = i + j;
            if partial_sum < 2020 {
                let difference = 2020 - partial_sum;
                if values.contains(&difference) {
                    return i * j * difference;
                }
            }
        }
    }

    panic!("No solution was found.");
}
