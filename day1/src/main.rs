use std::iter::zip;

use nom::{
    character::complete::{line_ending, space1, u32 as u32_parser},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn num_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32_parser, space1, u32_parser)(input)
}

fn parser(s: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, num_pair)(s)
}

fn solve(places: &[(u32, u32)]) -> u32 {
    // Get the distance between each sorted pair and add them up
    // sort
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = places.iter().map(|(f, s)| (f, s)).unzip();
    first.sort_unstable();
    second.sort_unstable();
    let sorted_places = zip(first, second);
    // add up diffs
    sorted_places.fold(0, |a, p| a + p.0.abs_diff(p.1))
}

fn solve2(places: &[(u32, u32)]) -> u32 {
    // Figure out how many times each number from the first list appears in the second list,
    // multiply that by each number in first list, then add them all together
    // pull apart
    let (first, second): (Vec<u32>, Vec<u32>) = places.iter().map(|(f, s)| (f, s)).unzip();
    let results = first
        .iter()
        .map(|a| second.iter().filter(|&b| b == a).count() as u32 * *a);
    // add up results
    results.sum()
}

fn main() {
    let places = utils::load_puzzle_data(1, parser);
    let distance_sum = solve(&places);
    println!("Solution 1: Distance sum: {}", distance_sum);

    let multiplied_sum = solve2(&places);
    println!("Solution 2: Multiplied sum: {}", multiplied_sum);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(1, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 11);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(1, 2, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 31);
    }
}
