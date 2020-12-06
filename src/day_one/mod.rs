use std::collections::HashSet;
use std::fs;

pub fn solve_day_one() {
    let values_set = parse_input();
    println!("Part one: {}", part_one(&values_set));
    println!("Part two: {}", part_two(&values_set));
}

/**
 * Part one is simple - we're looking for two numbers that
 * add up to a target sum.  We've already got a set of all the
 * numbers for quick lookups, so iterate over it and for each number
 * test to see if 2020 - x is in the set.
 */
fn part_one(values_set: &HashSet<i64>) -> i64 {
    for value in values_set {
        let difference = 2020 - *value;
        if values_set.contains(&difference) {
            return difference * value;
        }
    }

    panic!("No solution was found.")
}

/**
 * Part two is exactly the same, except we're looking for a
 * three-value sum.  We can do this with the same algorithm as part one,
 * except now we're checking to see if 2020 minus each pair of
 * values (i, j) in the set exists.  
 */
fn part_two(values_set: &HashSet<i64>) -> i64 {
    for i in values_set {
        for j in values_set {
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

fn parse_input() -> HashSet<i64> {
    let contents = fs::read_to_string("inputs/day-one.txt").expect("Oh no");
    let mut values = HashSet::new();
    for line in contents.lines() {
        let value = line.parse::<i64>().unwrap();
        values.insert(value);
    }

    values
}
