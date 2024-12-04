use nom::{
    character::complete::{i32 as i32_parser, line_ending, space1},
    multi::separated_list1,
    IResult,
};

enum LevelChange {
    Increasing,
    Decreasing,
    Unknown,
}

fn parse_levels(input: &str) -> IResult<&str, Vec<i32>> {
    // space delimited, any number, at least 1
    separated_list1(space1, i32_parser)(input)
}

fn parser(s: &str) -> IResult<&str, Vec<Vec<i32>>> {
    // Parse reports (lines), into levels (space delimited numbers)
    separated_list1(line_ending, parse_levels)(s)
}

fn is_report_safe(report: &[i32]) -> (bool, Vec<bool>) {
    let mut level_change: LevelChange = LevelChange::Unknown;
    let safe: Vec<bool> = report
        .windows(2)
        .map(|level_pair| {
            // They cannot be equal
            if level_pair[0] == level_pair[1] {
                return false;
            }
            // They must all be increasing or decreasing
            let decreasing = (level_pair[0] - level_pair[1]) > 0;
            match level_change {
                LevelChange::Increasing => {
                    if decreasing {
                        return false;
                    }
                }
                LevelChange::Decreasing => {
                    if !decreasing {
                        return false;
                    }
                }
                LevelChange::Unknown => {
                    // First iteration, determine what it should be
                    if decreasing {
                        level_change = LevelChange::Decreasing;
                    } else {
                        level_change = LevelChange::Increasing
                    }
                }
            }
            let diff = level_pair[0].abs_diff(level_pair[1]);
            // Must be at least 1, but not more than 3
            (1..=3).contains(&diff)
        })
        .collect();
    (safe.iter().all(|b| *b), safe)
}

fn solve(reports: &[Vec<i32>]) -> i32 {
    reports.iter().fold(
        0,
        |a, report| {
            if is_report_safe(report).0 {
                a + 1
            } else {
                a
            }
        },
    )
}

fn solve2(reports: &[Vec<i32>]) -> i32 {
    reports.iter().fold(0, |a, report| {
        let (is_safe, results) = is_report_safe(report);
        if is_safe {
            // Safe without removing anything
            return a + 1;
        } else {
            let remove_and_test = |r: &mut Vec<i32>, ndx_to_remove: usize| -> bool {
                let removed_index_val = r.remove(ndx_to_remove);
                let safe = is_report_safe(r).0;
                r.insert(ndx_to_remove, removed_index_val);
                safe
            };
            // Try removing one of the bad ones and solve again
            let mut new_report = report.clone();
            // There's a case where index 0 is bad, and it sets a bad level_change, try removing it first
            let safe = remove_and_test(&mut new_report, 0);
            if safe {
                return a + 1;
            }
            // Try removing the left of the first (and should be only) false result
            let safe = remove_and_test(&mut new_report, results.iter().position(|b| !*b).unwrap());
            if safe {
                return a + 1;
            }
            // Try removing the right of the first (and should be only) false result
            let safe = remove_and_test(
                &mut new_report,
                results.iter().position(|b| !*b).unwrap() + 1,
            );
            if safe {
                return a + 1;
            }
        }
        // Couldn't find a way to make the report safe
        a
    })
}

fn main() {
    let reports = utils::load_puzzle_data(2, parser);
    let num_safe_reports = solve(&reports);
    println!("Solution 1: There are {} safe reports.", num_safe_reports);

    let num_safe_reports = solve2(&reports);
    println!("Solution 2: There are {} safe reports.", num_safe_reports);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(2, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(2, 2, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 6);
    }
}
