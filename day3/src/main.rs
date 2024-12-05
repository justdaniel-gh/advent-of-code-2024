use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::u32 as u32_parser,
    combinator::map,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};
use utils::drop_until;

// Part 1
fn parse_mul_params(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32_parser, tag(","), u32_parser)(input)
}

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(tag("mul("), parse_mul_params, tag(")"))(input)
}

// Returns Vec<(first number, second number)>
fn parser(s: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many1(drop_until(parse_mul))(s)
}

fn parse_mul_params2(input: &str) -> IResult<&str, ParsedValue> {
    nom::combinator::map(separated_pair(u32_parser, tag(","), u32_parser), |v| {
        ParsedValue::Multiply(v)
    })(input)
}

// Part 2
enum ParsedValue {
    Multiply((u32, u32)),
    Do,
    Dont,
}

fn parse_mul2(input: &str) -> IResult<&str, ParsedValue> {
    delimited(tag("mul("), parse_mul_params2, tag(")"))(input)
}

fn parse_do(input: &str) -> IResult<&str, ParsedValue> {
    nom::combinator::map(tag("do()"), |_| ParsedValue::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, ParsedValue> {
    nom::combinator::map(tag("don't()"), |_| ParsedValue::Dont)(input)
}

// Returns Vec<(first number, second number)>
fn parser2(s: &str) -> IResult<&str, Vec<ParsedValue>> {
    many1(map(
        many_till(take(1u8), alt((parse_mul2, parse_do, parse_dont))),
        |(_, matched)| matched,
    ))(s)
}

fn solve(multiply_pairs: &[(u32, u32)]) -> u32 {
    // multiply and add!
    multiply_pairs.iter().fold(0, |a, p| a + (p.0 * p.1))
}

fn solve2(parsed_values: &[ParsedValue]) -> u32 {
    // Start with multiply enabled, then turn off on don'ts and back on with dos
    let mut do_multiply: bool = true;
    let mut accumulator = 0;
    for value in parsed_values {
        match value {
            ParsedValue::Multiply((first, second)) => {
                if do_multiply {
                    accumulator += first * second;
                }
            }
            ParsedValue::Do => do_multiply = true,
            ParsedValue::Dont => do_multiply = false,
        }
    }
    accumulator
}

fn main() {
    let values_to_multiply = utils::load_puzzle_data(3, parser);
    let product_sums = solve(&values_to_multiply);
    println!(
        "Solution 1: The sum of the multiplied values is {}",
        product_sums
    );

    let parsed_values = utils::load_puzzle_data(3, parser2);
    let product_sums = solve2(&parsed_values);
    println!(
        "Solution 2: The sum of the multiplied values, following dos and don'ts is {}",
        product_sums
    );
}

#[cfg(test)]
mod tests {
    use crate::{parser, parser2, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(3, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 161);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(3, 2, parser2);
        let solution = solve2(&test_data);
        assert_eq!(solution, 48);
    }
}
