#[derive(Copy, Clone, Debug, PartialEq)]
enum Pipe {
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    Ground,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            '.' => Self::Ground,
            _ => panic!("invalid pipe {}", c),
        }
    }
}

struct LoopStart {
    start: (usize, usize),
    map: Vec<Vec<Pipe>>,
}

impl From<Vec<Vec<Pipe>>> for LoopStart {
    fn from(map: Vec<Vec<Pipe>>) -> Self {
        let start = find_start(&map);
        Self { start, map }
    }
}

impl IntoIterator for LoopStart {
    type IntoIter = Loop;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        Loop {
            current: self.start,
            previous: self.start,
            distance: 0,
            map: self.map,
        }
    }
}

struct Loop {
    current: (usize, usize),
    previous: (usize, usize),
    distance: usize,
    map: Vec<Vec<Pipe>>,
}

// fn print_map(map: &Vec<Vec<Pipe>>, current: (usize, usize)) {
//     for (y, row) in map.iter().enumerate() {
//         for (x, pipe) in row.iter().enumerate() {
//             if (x, y) == current {
//                 eprint!("\x1b[31;1;4m");
//             }
//             match pipe {
//                 Pipe::Horizontal => eprint!("-"),
//                 Pipe::Vertical => eprint!("|"),
//                 Pipe::NorthEast => eprint!("L"),
//                 Pipe::NorthWest => eprint!("J"),
//                 Pipe::SouthEast => eprint!("F"),
//                 Pipe::SouthWest => eprint!("7"),
//                 Pipe::Start => eprint!("S"),
//                 Pipe::Ground => eprint!("."),
//             }
//             if (x, y) == current {
//                 eprint!("\x1b[0m");
//             }
//         }
//         eprintln!();
//     }
//     eprint!("\x1b[0m");
//     eprintln!();
// }

impl Iterator for Loop {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.current;
        let pipe = self.map[y][x];

        // print_map(&self.map, self.current);

        let old = self.current;
        // assume we never have to deal with overflowing map boundaries
        // TODO: use directions to make this less verbose
        match pipe {
            Pipe::Start if self.distance == 0 => {
                match self.map[y][x + 1] {
                    Pipe::Horizontal | Pipe::NorthWest | Pipe::SouthWest => {
                        self.current = (x + 1, y);
                    },
                    _ => {}
                }
                if self.current == self.previous {
                    match self.map[y + 1][x] {
                        Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast => {
                            self.current = (x, y + 1);
                        },
                        _ => {}
                    }
                }
                if self.current == self.previous {
                    panic!("didn't find any pipes connecting to start!");
                }
            },
            Pipe::Start => {
                // back to start (distance > 0)
                return None;
            },
            Pipe::Horizontal => {
                if self.previous.0 < x {
                    // moving right
                    self.current = (x + 1, y);
                } else {
                    // moving left
                    self.current = (x - 1, y);
                }
            },
            Pipe::Vertical => {
                if self.previous.1 < y {
                    // moving down
                    self.current = (x, y + 1);
                } else {
                    // moving up
                    self.current = (x, y - 1);
                }
            },
            Pipe::NorthEast => {
                if self.previous.1 < y {
                    // moving right
                    self.current = (x + 1, y);
                } else {
                    // moving up 
                    self.current = (x, y - 1);
                }
            },
            Pipe::NorthWest => {
                if self.previous.1 < y {
                    // moving left
                    self.current = (x - 1, y);
                } else {
                    // moving up
                    self.current = (x, y - 1);
                }
            },
            Pipe::SouthEast => {
                if self.previous.1 > y {
                    // moving right
                    self.current = (x + 1, y);
                } else {
                    // moving down
                    self.current = (x, y + 1);
                }
            },
            Pipe::SouthWest => {
                if self.previous.1 > y {
                    // moving left
                    self.current = (x - 1, y);
                } else {
                    // moving down
                    self.current = (x, y + 1);
                }
            },
            Pipe::Ground => {
                panic!("your pipe is leaking");
            },
        }
        self.previous = old;

        self.distance += 1;
        Some(self.distance)
    }
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input.lines().map(|line| line.chars().map(From::from).collect()).collect()
}

fn find_start(map: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if *pipe == Pipe::Start {
                return (x, y);
            }
        }
    }
    panic!("no start found");
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let loop_start = LoopStart::from(map);
    loop_start.into_iter().last().expect("loop") / 2
}

pub fn part2(input: &str) -> usize {
    todo!()
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), None)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    #[test]
    fn day10_sample_part1() {
        assert_eq!(part1(SAMPLE), 8);
    }

    #[ignore]
    #[test]
    fn day10_sample_part2() {
        assert_eq!(part2(SAMPLE), 2);
    }
}

