use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    distance: isize,
}

mod parse {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::take_while_m_n,
        character::complete::{anychar, char, digit1, space1},
        combinator::{map, map_res},
        multi::many0,
        sequence::{delimited, pair, preceded, tuple},
        IResult,
    };

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        alt((
            map(char('U'), |_| Direction::Up),
            map(char('D'), |_| Direction::Down),
            map(char('L'), |_| Direction::Left),
            map(char('R'), |_| Direction::Right),
        ))(input)
    }

    fn parse_distance(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn from_hex(input: &str) -> Result<usize, std::num::ParseIntError> {
        usize::from_str_radix(input, 16)
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_digit(16)
    }

    fn parse_part2_distance(input: &str) -> IResult<&str, usize> {
        map_res(take_while_m_n(5, 5, is_hex_digit), from_hex)(input)
    }

    fn parse_part2_direction(input: &str) -> IResult<&str, Direction> {
        alt((
            map(char('3'), |_| Direction::Up),
            map(char('1'), |_| Direction::Down),
            map(char('2'), |_| Direction::Left),
            map(char('0'), |_| Direction::Right),
        ))(input)
    }

    fn parse_part2_hex(input: &str) -> IResult<&str, Instruction> {
        pair(
            preceded(char('#'), parse_part2_distance),
            parse_part2_direction,
        )(input)
        .map(|(input, (distance, direction))| {
            (
                input,
                Instruction {
                    distance: distance as isize,
                    direction,
                },
            )
        })
    }

    pub fn parse_instruction_part_1(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((parse_direction, space1, parse_distance, many0(anychar))),
            |(direction, _, distance, _)| Instruction {
                direction,
                distance: distance as isize,
            },
        )(input)
    }

    pub fn parse_instruction_part_2(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                parse_direction,
                space1,
                parse_distance,
                space1,
                delimited(char('('), parse_part2_hex, char(')')),
            )),
            |(_, _, _, _, instruction)| instruction,
        )(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn can_parse_direction() {
            assert_eq!(parse_direction("U"), Ok(("", Direction::Up)));
            assert_eq!(parse_direction("D"), Ok(("", Direction::Down)));
            assert_eq!(parse_direction("L"), Ok(("", Direction::Left)));
            assert_eq!(parse_direction("R"), Ok(("", Direction::Right)));
        }

        #[test]
        fn can_parse_distance() {
            assert_eq!(parse_distance("1"), Ok(("", 1)));
            assert_eq!(parse_distance("12"), Ok(("", 12)));
            assert_eq!(parse_distance("123"), Ok(("", 123)));
        }

        #[test]
        fn can_parse_instruction() {
            assert_eq!(
                parse_instruction_part_1("U 1 (#000000)"),
                Ok((
                    "",
                    Instruction {
                        direction: Direction::Up,
                        distance: 1,
                    }
                ))
            );
            assert_eq!(
                parse_instruction_part_1("D 12 (#ffffff)"),
                Ok((
                    "",
                    Instruction {
                        direction: Direction::Down,
                        distance: 12,
                    }
                ))
            );
            assert_eq!(
                parse_instruction_part_1("L 123 (#ff0000)"),
                Ok((
                    "",
                    Instruction {
                        direction: Direction::Left,
                        distance: 123,
                    }
                ))
            );
            assert_eq!(
                parse_instruction_part_1("R 1234 (#00ff00)"),
                Ok((
                    "",
                    Instruction {
                        direction: Direction::Right,
                        distance: 1234,
                    }
                ))
            );
        }

        #[test]
        fn can_parse_instruction_part_2() {
            assert_eq!(
                parse_instruction_part_2("R 6 (#70c710)"),
                Ok((
                    "",
                    Instruction {
                        direction: Direction::Right,
                        distance: 461937,
                    }
                ))
            );
        }
    }
}

fn parse_input_part1(input: &str) -> Vec<Instruction> {
    use parse::parse_instruction_part_1;

    input
        .par_lines()
        .map(|line| {
            parse_instruction_part_1(line)
                .expect("can parse instruction")
                .1
        })
        .collect()
}

fn parse_input_part2(input: &str) -> Vec<Instruction> {
    use parse::parse_instruction_part_2;

    input
        .par_lines()
        .map(|line| {
            parse_instruction_part_2(line)
                .expect("can parse instruction")
                .1
        })
        .collect()
}

fn generate_polygon(instructions: Vec<Instruction>) -> Vec<(isize, isize)> {
    instructions
        .into_iter()
        .fold(vec![(0isize, 0isize)], |mut polygon, instruction| {
            let (mut x, mut y) = polygon.last().expect("polygon has at least one point");
            match instruction.direction {
                Direction::Up => y -= instruction.distance,
                Direction::Down => y += instruction.distance,
                Direction::Left => x -= instruction.distance,
                Direction::Right => x += instruction.distance,
            };

            polygon.push((x, y));
            polygon
        })
}

fn shoelace(polygon: &Vec<(isize, isize)>) -> usize {
    let perimeter = polygon
        .iter()
        .zip(polygon.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| (y2 - y1).abs() + (x2 - x1).abs())
        .sum::<isize>() as usize;

    let double_area = polygon
        .iter()
        .zip(polygon.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| (x1 * y2) - (x2 * y1))
        .sum::<isize>()
        .abs() as usize
        + perimeter;
    (double_area / 2) + 1
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input_part1(input);
    let polygon = generate_polygon(instructions);
    shoelace(&polygon)
}

pub fn part2(input: &str) -> usize {
    let instructions = parse_input_part2(input);
    let polygon = generate_polygon(instructions);
    shoelace(&polygon)
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn day17_sample_part1() {
        assert_eq!(part1(SAMPLE), 62);
    }

    #[test]
    fn day17_sample_part2() {
        assert_eq!(part2(SAMPLE), 952408144115);
    }
}
