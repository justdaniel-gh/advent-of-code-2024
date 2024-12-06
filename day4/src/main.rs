use nom::{
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    IResult,
};
use utils::{Grid, StaticGrid};

fn parser(s: &str) -> IResult<&str, StaticGrid<char>> {
    // Odd way of going about it, I know, but it ensures it parses
    let rows = separated_list1(line_ending, alpha1)(s)?;
    let grid: StaticGrid<char> = StaticGrid {
        cells: rows.1.concat().chars().collect(),
        num_rows: rows.1.len(),
        num_cols: rows.1[0].len(),
    };
    Ok(("", grid))
}

fn solve(grid: &StaticGrid<char>) -> u32 {
    // Find all of the word "XMAS" in the grid
    let mut count = 0;
    for y in 0..grid.num_rows as isize {
        for x in 0..grid.num_cols as isize {
            // Only start at X's
            if grid.get_cell(x, y) != Some(&'X') {
                continue;
            }
            // Check every direction for XMAS
            for direction in enum_iterator::all::<utils::CardinalDirection>() {
                if grid
                    .direction_iter_at(x, y, direction)
                    .take(4)
                    .collect::<String>()
                    .as_str()
                    == "XMAS"
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve2(grid: &StaticGrid<char>) -> u32 {
    // Find all "MAS" in an X pattern
    let mut count = 0;
    for y in 0..grid.num_rows as isize {
        for x in 0..grid.num_cols as isize {
            // Only start at A's (the center of the X)
            if grid.get_cell(x, y) != Some(&'A') {
                continue;
            }
            // Check NE and SW, one must be a 'S' and the other a 'M'
            // Do the same for NW and SE
            let ne = grid
                .direction_iter_at(x, y, utils::CardinalDirection::NorthEast)
                .skip(1).take(1).collect::<Vec<&char>>();
            let sw = grid
                .direction_iter_at(x, y, utils::CardinalDirection::SouthWest)
                .skip(1).take(1).collect::<Vec<&char>>();
            let nw = grid
                .direction_iter_at(x, y, utils::CardinalDirection::NorthWest)
                .skip(1).take(1).collect::<Vec<&char>>();
            let se = grid
                .direction_iter_at(x, y, utils::CardinalDirection::SouthEast)
                .skip(1).take(1).collect::<Vec<&char>>();
            if ((ne.first() == Some(&&'M') && sw.first() == Some(&&'S')) || (ne.first() == Some(&&'S') && sw.first() == Some(&&'M')))
                && ((nw.first() == Some(&&'M') && se.first() == Some(&&'S')) || (nw.first() == Some(&&'S') && se.first() == Some(&&'M')))
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let grid = utils::load_puzzle_data(4, parser);
    let number_of_xmas = solve(&grid);
    println!(
        "Solution 1: There are {} XMAS in the word search.",
        number_of_xmas
    );

    let number_of_xmas = solve2(&grid);
    println!(
        "Solution 2: There are {} X-MAS in the word search.",
        number_of_xmas
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(4, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 18);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(4, 1, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 9);
    }
}
