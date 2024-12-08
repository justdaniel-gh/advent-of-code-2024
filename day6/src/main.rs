use core::num;
use std::fmt;

use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use utils::{CardinalDirection, Point, StaticGrid};

#[derive(Default, Clone, Debug)]
struct Cell {
    visited: bool,
    visited_dir: Vec<CardinalDirection>,
    obstruction: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.visited {
            if self.visited_dir.len() == 1 {
                let x = match self.visited_dir[0] {
                    CardinalDirection::North => "^",
                    CardinalDirection::East => ">",
                    CardinalDirection::South => "v",
                    CardinalDirection::West => "<",
                    _ => "#",
                };
                write!(f, "{x}")
            } else {
                write!(f, "+")
            }
        } else if self.obstruction {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

struct Game {
    starting_position: Point,
    starting_direction: CardinalDirection,
    player_position: Point,
    player_direction: CardinalDirection,
    grid: StaticGrid<Cell>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = String::new();

        for row_ndx in 0..self.grid.num_rows {
            let row_str: String = self
                .grid
                .row(row_ndx)
                .iter()
                .enumerate()
                .map(|(c_ndx, c)| {
                    if self.player_position.x == c_ndx as isize
                        && self.player_position.y == row_ndx as isize
                    {
                        match self.player_direction {
                            CardinalDirection::North => "^".to_owned(),
                            CardinalDirection::East => ">".to_owned(),
                            CardinalDirection::South => "v".to_owned(),
                            CardinalDirection::West => "<".to_owned(),
                            _ => "?".to_owned(),
                        }
                    } else {
                        ToString::to_string(c)
                    }
                })
                .collect();
            rows.push_str(&row_str);
            rows.push('\n');
        }
        write!(f, "{rows}")
    }
}

enum CellVariant {
    Player(CardinalDirection),
    Obstruction,
    EmptySpace,
}

fn parse_cell(s: &str) -> IResult<&str, CellVariant> {
    map(one_of(".#^<>v"), |c| {
        match c {
            '#' => CellVariant::Obstruction,
            '^' => CellVariant::Player(CardinalDirection::North),
            '<' => CellVariant::Player(CardinalDirection::West),
            '>' => CellVariant::Player(CardinalDirection::East),
            'v' => CellVariant::Player(CardinalDirection::North),
            // "."
            _ => CellVariant::EmptySpace,
        }
    })(s)
}

fn parse_cell_row(s: &str) -> IResult<&str, Vec<CellVariant>> {
    many1(parse_cell)(s)
}

fn parser(s: &str) -> IResult<&str, Game> {
    // Odd way of going about it, I know, but it ensures it parses
    let rows = separated_list1(line_ending, parse_cell_row)(s)?;
    let mut player_position = Point::new(0, 0);
    let mut player_direction = CardinalDirection::North;
    let grid = StaticGrid {
        cells: rows
            .1
            .iter()
            .enumerate()
            .flat_map(|(row_ndx, cvs)| {
                cvs.iter()
                    .enumerate()
                    .map(|(col_ndx, cv)| match cv {
                        CellVariant::Player(cardinal_direction) => {
                            player_position = Point::new(col_ndx as isize, row_ndx as isize);
                            player_direction = *cardinal_direction;
                            Cell {
                                visited: true,
                                visited_dir: vec![*cardinal_direction],
                                ..Default::default()
                            }
                        }
                        CellVariant::Obstruction => Cell {
                            obstruction: true,
                            ..Default::default()
                        },
                        CellVariant::EmptySpace => Cell {
                            ..Default::default()
                        },
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect(),
        num_rows: rows.1.len(),
        num_cols: rows.1[0].len(),
    };

    let game = Game {
        starting_position: player_position,
        starting_direction: player_direction,
        player_position,
        player_direction,
        grid,
    };

    Ok(("", game))
}

fn solve(game: &mut Game) -> usize {
    // Play the game... just move the player around!
    let mut playing = true;
    while playing {
        let mut it = game
            .grid
            .direction_iter_at_mut(
                game.player_position.x,
                game.player_position.y,
                game.player_direction,
            )
            .skip(1) // Skip the current cell we're on
            .peekable();
        while let Some(c) = it.next() {
            if c.obstruction {
                game.player_direction = game
                    .player_direction
                    .rotate_by_angle(&utils::RotateAmount::_90);
                break;
            }
            c.visited = true;
            c.visited_dir.push(game.player_direction);
            game.player_position.add(1, game.player_direction);

            if it.peek().is_none() {
                playing = false;
                break;
            }
        }
    }
    println!("{game}");
    game.grid.cell_iter().filter(|c| c.visited).count()
}

fn solve2(game: &mut Game) -> usize {
    // We have a solved game. Just find every cross section
    // where placing an obstruction would cause the guard to
    // turn in the direction already traveled.
    // First solve the game to get the paths, then just run through it (this is all so I don't have to use a mutating iterator)
    println!("{game}");
    solve(game);
    // Reset player
    game.player_position = game.starting_position;
    game.player_direction = game.starting_direction;
    println!("{game}");
    let mut playing = true;
    let mut number_of_obstructions = 0;
    let grid = &mut game.grid;
    while playing {
        let mut it = grid
            .direction_iter_at_mut(
                game.player_position.x,
                game.player_position.y,
                game.player_direction,
            )
            .skip(1) // Skip the current cell we're on
            .peekable();
        while let Some(c) = it.next() {
            /*
            This solution only found about half of them. What about times when we aren't about to cross ourselves,
            but placing it would put us in line with an existing path.
            if c.visited {
                // Crossing an already visited block
                // If we placed an obstruction, would it cause us to turn into the direction
                //  already traveled?
                let new_dir = game.player_direction.rotate_by_angle(&utils::RotateAmount::_90);
                let v_dir = &c.visited_dir;
                let pos = (game.player_position.x, game.player_position.y);
                println!("vdir: {v_dir:?} new_dir: {new_dir:?} pos: {pos:?}, amt moved: {amt_moved}");
                if c.visited_dir.first() == Some(&new_dir) {
                    // Yes! Count it
                    number_of_obstructions += 1;
                    println!("This'll work");
                }
            }
             */
            /* Instead, every move forward, check to see if turning 90deg, and following that, would intersect a visited
              block that is moving in the same direction.
            */
            // If this block is an obstruction, don't check it
            if c.obstruction {
                break;
            }
            // Rotate 90 right here and see if we can intersect a path
            let possible_dir = game
                .player_direction
                .rotate_by_angle(&utils::RotateAmount::_90);
            let v_dir = &c.visited_dir;
            let pos = (game.player_position.x, game.player_position.y);
            println!("vdir: {v_dir:?} new_dir: {possible_dir:?} pos: {pos:?}");
            let mut possible_it = grid
                .direction_iter_at(game.player_position.x, game.player_position.y, possible_dir)
                .skip(1) // Skip the current cell we're on
                .peekable();
            while let Some(pc) = possible_it.next() {
                if pc.obstruction {
                    break;
                }
                // We found an entrance to a loop
                if pc.visited_dir.iter().find(|&v| v == &possible_dir).is_some() {
                    number_of_obstructions += 1;
                    println!("Found one.");
                    break;
                }
            }

            // Move the player...
            game.player_position.add(1, game.player_direction);

            // If we're at the edge of the board...
            if it.peek().is_none() {
                playing = false;
                break;
            }
        }
        game.player_direction = game
            .player_direction
            .rotate_by_angle(&utils::RotateAmount::_90);
    }
    number_of_obstructions
}

fn main() {
    let mut game = utils::load_puzzle_data(6, parser);
    let num_visited_positions = solve(&mut game);
    println!(
        "Solution 1: The guard visited {} positions.",
        num_visited_positions
    );

    // Reset the game
    let mut game = utils::load_puzzle_data(6, parser);
    let num_obstructions_placed = solve2(&mut game);
    println!(
        "Solution 2: There are {} places to put an obstacle to keep the guard in a loop.",
        num_obstructions_placed
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let mut test_data = utils::load_puzzle_test(6, 1, parser);
        let solution = solve(&mut test_data);
        assert_eq!(solution, 41);
    }

    #[test]
    fn test_puzzle2() {
        let mut test_data = utils::load_puzzle_test(6, 1, parser);
        let solution = solve2(&mut test_data);
        assert_eq!(solution, 6);
    }
}
