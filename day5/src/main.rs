#![feature(int_roundings)]

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32 as u32_parser},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

type PuzzleInput = (Vec<(u32, u32)>, Vec<Vec<u32>>);

fn num_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32_parser, tag("|"), u32_parser)(input)
}

fn parser(s: &str) -> IResult<&str, PuzzleInput> {
    /*
    D+|D+\n
    ...\n
    \n
    D+,...\n
    ...
    */
    separated_pair(
        separated_list1(line_ending, num_pair),
        many1(line_ending),
        separated_list1(line_ending, separated_list1(tag(","), u32_parser)),
    )(s)
}

fn solve(input: &PuzzleInput) -> usize {
    // Search through each update
    let ordered_updates: Vec<&Vec<u32>> = input
        .1
        .iter()
        .filter(|&update| {
            update.iter().enumerate().all(|(page_ndx, page)| {
                // Is this page in a rule?
                input
                    .0
                    .iter()
                    .filter(|&rule| rule.0 == *page)
                    .all(|matched_rule| {
                        match update.iter().position(|&m| m == matched_rule.1) {
                            Some(found_page_ndx) => found_page_ndx > page_ndx,
                            // It's in order if the number doesn't exist
                            None => true,
                        }
                    })
            })
        })
        .collect();

    // Find the middles, add them up
    ordered_updates.iter().fold(0, |a, &update| {
        a + *update.get(update.len().div_floor(2)).unwrap() as usize
    })
}

fn solve2(input: &mut PuzzleInput) -> usize {
    // Search through each update, finding invalid ones
    let mut un_ordered_updates: Vec<&mut Vec<u32>> = input
        .1
        .iter_mut()
        .filter(|update| {
            !update.iter().enumerate().all(|(page_ndx, page)| {
                // Is this page in a rule?
                input
                    .0
                    .iter()
                    .filter(|&rule| rule.0 == *page)
                    .all(|matched_rule| {
                        match update.iter().position(|&m| m == matched_rule.1) {
                            Some(found_page_ndx) => found_page_ndx > page_ndx,
                            // It's in order if the number doesn't exist
                            None => true,
                        }
                    })
            })
        })
        .collect();

    // Now order them...
    for update in un_ordered_updates.iter_mut() {
        update.sort_by(|a, b| {
            // Is there a sort rule for these numbers?
            match input
                .0
                .iter()
                .find(|&rule| (rule.0 == *a && rule.1 == *b) || (rule.1 == *a && rule.0 == *b))
            {
                Some(a_rule) => {
                    if a_rule.0 == *a {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }
                None => std::cmp::Ordering::Equal,
            }
        });
    }

    // Find the middles, add them up
    un_ordered_updates.iter().fold(0, |a, update| {
        a + *update.get(update.len().div_floor(2)).unwrap() as usize
    })
}

fn main() {
    let mut places = utils::load_puzzle_data(5, parser);
    let middle_sum = solve(&places);
    println!(
        "Solution 1: The sum of the middle page of valid updates is {}",
        middle_sum
    );

    let middle_sum = solve2(&mut places);
    println!(
        "Solution 1: The sum of the middle page of corrected updates is {}",
        middle_sum
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(5, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 143);
    }

    #[test]
    fn test_puzzle2() {
        let mut test_data = utils::load_puzzle_test(5, 1, parser);
        let solution = solve2(&mut test_data);
        assert_eq!(solution, 123);
    }
}
