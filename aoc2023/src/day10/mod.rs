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
    type Item = LoopStep;

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

struct LoopStep {
    coords: (usize, usize),
    distance: usize,
}

impl Iterator for Loop {
    type Item = LoopStep;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.current;
        let pipe = self.map[y][x];

        let old = self.current;
        // assume we never have to deal with overflowing map boundaries
        // TODO: use directions to make this less verbose
        match pipe {
            Pipe::Start if self.distance == 0 => {
                match self.map[y][x + 1] {
                    Pipe::Horizontal | Pipe::NorthWest | Pipe::SouthWest => {
                        self.current = (x + 1, y);
                    }
                    _ => {}
                }
                if self.current == self.previous {
                    match self.map[y + 1][x] {
                        Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast => {
                            self.current = (x, y + 1);
                        }
                        _ => {}
                    }
                }
                if self.current == self.previous {
                    panic!("didn't find any pipes connecting to start!");
                }
            }
            Pipe::Start => {
                // back to start (distance > 0)
                return None;
            }
            Pipe::Horizontal => {
                if self.previous.0 < x {
                    // moving right
                    self.current = (x + 1, y);
                } else {
                    // moving left
                    self.current = (x - 1, y);
                }
            }
            Pipe::Vertical => {
                if self.previous.1 < y {
                    // moving down
                    self.current = (x, y + 1);
                } else {
                    // moving up
                    self.current = (x, y - 1);
                }
            }
            Pipe::NorthEast => {
                if self.previous.1 < y {
                    // moving right
                    self.current = (x + 1, y);
                } else {
                    // moving up
                    self.current = (x, y - 1);
                }
            }
            Pipe::NorthWest => {
                if self.previous.1 < y {
                    // moving left
                    self.current = (x - 1, y);
                } else {
                    // moving up
                    self.current = (x, y - 1);
                }
            }
            Pipe::SouthEast => {
                if self.previous.1 > y {
                    // moving right
                    self.current = (x + 1, y);
                } else {
                    // moving down
                    self.current = (x, y + 1);
                }
            }
            Pipe::SouthWest => {
                if self.previous.1 > y {
                    // moving left
                    self.current = (x - 1, y);
                } else {
                    // moving down
                    self.current = (x, y + 1);
                }
            }
            Pipe::Ground => {
                panic!("your pipe is leaking");
            }
        }
        self.previous = old;

        self.distance += 1;
        Some(LoopStep {
            coords: self.current,
            distance: self.distance,
        })
    }
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(From::from).collect())
        .collect()
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
    loop_start
        .into_iter()
        .last()
        .map(|s| s.distance)
        .expect("loop")
        / 2
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);
    let start = find_start(&map);
    let mut pipe: Vec<Vec<Option<usize>>> = vec![vec![None; map[0].len()]; map.len()];
    pipe[start.1][start.0] = Some(0);
    let loop_start = LoopStart::from(map);
    let mut max_distance = 0;
    for step in loop_start.into_iter() {
        let (x, y) = step.coords;
        pipe[y][x] = Some(step.distance);
        max_distance = step.distance;
    }

    let mut inside_count = 0;
    for y in 0..pipe.len() {
        let mut winding: isize = 0;
        for x in 0..pipe[y].len() {
            if pipe[y][x].is_none() {
                if winding != 0 {
                    inside_count += 1;
                }
                continue;
            }
            let d_this = pipe[y][x].unwrap();

            if y + 1 >= pipe.len() {
                continue;
            }
            if let Some(d_below) = pipe[y + 1][x] {
                let dy = (d_this as isize - d_below as isize) % max_distance as isize;
                if dy == 1 {
                    // moved down
                    winding += 1;
                } else if dy == -1 {
                    // moved up
                    winding -= 1;
                }
            }
        }
    }

    inside_count
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
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

    const SAMPLE1: &'static str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
    const SAMPLE2: &'static str = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;
    const SAMPLE3: &'static str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
    const SAMPLE4: &'static str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn day10_sample1_part2() {
        assert_eq!(part2(SAMPLE1), 4);
    }

    #[test]
    fn day10_sample2_part2() {
        assert_eq!(part2(SAMPLE2), 4);
    }

    #[test]
    fn day10_sample3_part2() {
        assert_eq!(part2(SAMPLE3), 8);
    }

    #[test]
    fn day10_sample4_part2() {
        assert_eq!(part2(SAMPLE4), 10);
    }
}
