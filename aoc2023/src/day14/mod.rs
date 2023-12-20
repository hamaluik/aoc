use std::collections::HashMap;

use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Block,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Block,
            'O' => Tile::Rock,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn calculate_load(map: &Vec<Vec<Tile>>) -> usize {
    let (width, height) = (map[0].len(), map.len());
    (0..width)
        .into_par_iter()
        .map(|x| {
            let mut next_y = 0;
            let mut weighted_sum = 0;
            for y in 0..height {
                match map[y][x] {
                    Tile::Open => {}
                    Tile::Block => next_y = y + 1,
                    Tile::Rock => {
                        weighted_sum += height - next_y;
                        next_y += 1;
                    }
                }
            }
            weighted_sum
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);
    calculate_load(&map)
}

#[derive(Copy, Clone)]
enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

struct RotatableMap {
    map: Vec<Vec<Tile>>,
    size: usize,
}

impl std::fmt::Display for RotatableMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for tile in row {
                match tile {
                    Tile::Open => write!(f, ".")?,
                    Tile::Block => write!(f, "#")?,
                    Tile::Rock => write!(f, "O")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl RotatableMap {
    fn new(map: Vec<Vec<Tile>>) -> Self {
        let (width, height) = (map[0].len(), map.len());
        assert_eq!(width, height); // makes things easier
        let size = width;
        Self { map, size }
    }

    fn map_coords(&self, (x, y): (usize, usize), rotation: Rotation) -> (usize, usize) {
        match rotation {
            Rotation::Zero => (x, y),
            Rotation::Ninety => (self.size - y - 1, x),
            Rotation::OneEighty => (x, self.size - y - 1),
            Rotation::TwoSeventy => (y, self.size - x - 1),
        }
    }

    fn get(&self, (x, y): (usize, usize), rotation: Rotation) -> Tile {
        let (x, y) = self.map_coords((x, y), rotation);
        self.map[y][x]
    }

    fn set(&mut self, (x, y): (usize, usize), rotation: Rotation, tile: Tile) {
        let (x, y) = self.map_coords((x, y), rotation);
        self.map[y][x] = tile;
    }

    fn tilt(&mut self, rotation: Rotation) {
        for x in 0..self.size {
            let mut next_y = 0;
            for y in 0..self.size {
                match self.get((x, y), rotation) {
                    Tile::Open => {}
                    Tile::Block => next_y = y + 1,
                    Tile::Rock => {
                        self.set((x, y), rotation, Tile::Open);
                        self.set((x, next_y), rotation, Tile::Rock);
                        next_y += 1;
                    }
                }
            }
        }
    }
}

fn spin_cycle(map: &mut RotatableMap) {
    map.tilt(Rotation::Zero);
    map.tilt(Rotation::TwoSeventy);
    map.tilt(Rotation::OneEighty);
    map.tilt(Rotation::Ninety);
}

fn hash_map(map: &Vec<Vec<Tile>>) -> usize {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    map.hash(&mut hasher);
    hasher.finish() as usize
}

// TODO: SO CLOSE!
pub fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut map = RotatableMap::new(map);

    let mut history: HashMap<usize, usize> = HashMap::new();

    let mut i = 0;
    loop {
        let hash = hash_map(&map.map);
        if history.contains_key(&hash) {
            // found the cycle
            let cycle_start = history[&hash];
            let cycle_length = i - cycle_start;
            let remaining = 1_000_000_000 - cycle_start;
            let remaining = remaining % cycle_length;
            for _ in 0..remaining {
                spin_cycle(&mut map);
            }
            return calculate_load(&map.map);
        } else {
            history.insert(hash, i);
            spin_cycle(&mut map);
            i += 1;
        }
    }
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), None)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn day14_sample_part1() {
        assert_eq!(part1(SAMPLE), 136);
    }

    #[test]
    #[ignore]
    fn day14_sample_part2() {
        assert_eq!(part2(SAMPLE), 64);
    }

    #[test]
    fn can_map_coords() {
        let map = RotatableMap::new(parse(SAMPLE));
        assert_eq!(map.map_coords((0, 0), Rotation::Zero), (0, 0));
        assert_eq!(map.map_coords((0, 9), Rotation::Zero), (0, 9));

        assert_eq!(map.map_coords((0, 0), Rotation::Ninety), (9, 0));
        assert_eq!(map.map_coords((9, 0), Rotation::Ninety), (9, 9));

        assert_eq!(map.map_coords((0, 0), Rotation::OneEighty), (0, 9));
        assert_eq!(map.map_coords((0, 9), Rotation::OneEighty), (0, 0));

        assert_eq!(map.map_coords((0, 0), Rotation::TwoSeventy), (0, 9));
        assert_eq!(map.map_coords((9, 0), Rotation::TwoSeventy), (0, 0));
    }

    #[test]
    fn can_tilt() {
        let mut map = RotatableMap::new(parse(SAMPLE));
        map.tilt(Rotation::Zero);
        assert_eq!(
            map.to_string(),
            r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
"#
        );

        map.tilt(Rotation::Ninety);
        assert_eq!(
            map.to_string(),
            r#".OOOO#...O
..OO#....#
..OOO##..O
..O#....OO
........#.
..#....#.#
....O#..OO
.........O
#....###..
#....#....
"#
        );

        map.tilt(Rotation::OneEighty);
        assert_eq!(
            map.to_string(),
            r#"...OO#...O
..OO#....#
..OO.##...
..O#....OO
..O.....#O
..#....#.#
.....#....
..........
#...O###.O
#O..O#..OO
"#
        );

        map.tilt(Rotation::TwoSeventy);
        assert_eq!(
            map.to_string(),
            r#"OO...#O...
OO..#....#
OO...##...
O..#OO....
O.......#O
..#....#.#
.....#....
..........
#O...###O.
#OO..#OO..
"#
        );
    }

    #[test]
    fn can_spin_cycle() {
        let mut map = RotatableMap::new(parse(SAMPLE));
        spin_cycle(&mut map);
        assert_eq!(
            map.to_string(),
            r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"#
        );
        spin_cycle(&mut map);
        assert_eq!(
            map.to_string(),
            r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"#
        );
        spin_cycle(&mut map);
        assert_eq!(
            map.to_string(),
            r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"#
        );
    }
}
