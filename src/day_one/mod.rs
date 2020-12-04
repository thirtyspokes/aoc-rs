use std::collections::HashSet;
use std::fs;

pub fn solve_day_one() {
    let (values_set, values_list) = parse_input();
    println!("Part one: {}", part_one(&values_set, &values_list));
    println!("Part two: {}", part_two(&values_set, &values_list));
}

fn part_one(values_set: &HashSet<i64>, values_list: &[i64]) -> i64 {
    for value in values_list {
        let difference = 2020 - *value;
        if values_set.contains(&difference) {
            return difference * value;
        }
    }

    panic!("No solution was found.")
}

fn part_two(values_set: &HashSet<i64>, values_list: &[i64]) -> i64 {
    for i in values_list {
        for j in values_list {
            let partial_sum = i + j;
            if partial_sum < 2020 {
                let difference = 2020 - partial_sum;
                if values_set.contains(&difference) {
                    return i * j * difference;
                }
            }
        }
    }

    panic!("No solution was found.");
}

fn parse_input() -> (HashSet<i64>, Vec<i64>) {
    let contents = fs::read_to_string("inputs/day-one.txt").expect("Oh no");
    let mut values = HashSet::new();
    let mut vector = Vec::new();
    for line in contents.lines() {
        let value = line.parse::<i64>().unwrap();
        values.insert(value);
        vector.push(value);
    }

    (values, vector)
}
