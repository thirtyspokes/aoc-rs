use std::collections::HashSet;
use std::fs;

pub fn solve_day_six() {
    let input = fs::read_to_string("inputs/day-six.txt").unwrap();
    let groups: Vec<&str> = input.split("\n\n").collect();

    // Part one
    println!(
        "Part one: the total number of unique answers is {}",
        solve_part_one(&groups)
    );

    // Part two
    println!(
        "Part two: the total number of shared answers in groups is {}",
        solve_part_two(&groups)
    );
}

/**
 * For this problem, we want to add up all of the unique
 * values present among the answers for each group, which are
 * provided as group of lines of strings.
 *
 * We can solve this by transforming it into a collection of
 * HashSets (where each item in the collection is a HashSet containing
 * just the unique answers for each group), and then summing the
 * length of those sets.
 */
fn solve_part_one(groups: &[&str]) -> usize {
    let unique_groups: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| group.replace("\n", "")) // Turn the group of lines into a single line
        .map(|line| line.chars().collect()) // Create a vector of the chars in the line
        .collect(); // Collect the whole group into a HashSet

    // Now we just need to sum up the length of each group's
    // set of unique answers.
    unique_groups.iter().map(|group| group.len()).sum()
}

/**
 * For part two, we want to know the total number of answers for all groups
 * where every member of a group answered a question: that is, how many answers
 * are present on all of the lines in the group?
 *
 * We should theoretically be able to do this easily by turning each group
 * into a collection of sets (instead of just one big set), and then counting the
 * size of the intersection between those sets.  Unfortunately rust makes this
 * a little weird.
 */
fn solve_part_two(groups: &[&str]) -> usize {
    let groups: Vec<Vec<HashSet<char>>> = groups
        .iter()
        .map(|group| group.split('\n')) // Split each group into lines
        // Then, collect each line into a HashSet of its chars
        .map(|split_group| split_group.map(|line| line.chars().collect()).collect())
        .collect(); // This produces a vec of vecs of HashSets.

    let mut sum = 0;

    for group in groups {
        // There's only one responder in this group, so
        // all of the questions were answered by 'everyone'.
        if group.len() == 1 {
            sum += group[0].len();
            continue;
        }

        // In essence, all we're doing here is folding over each set in the group,
        // intersecting it with the next set and so on to produce the final
        // intersection of all the sets in the group.
        //
        // Rust forces us to copy things around in order to do this which probably
        // isn't performant, but whatever.
        let intersect = group.iter().fold(None, |acc: Option<HashSet<char>>, set| {
            let set = set.iter().copied().collect();
            acc.map(|a| a.intersection(&set).copied().collect())
                .or(Some(set))
        });

        sum += intersect.unwrap().len()
    }

    sum
}
