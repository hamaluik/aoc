use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Empty,
    SplitVertical,
    SplitHorizontal,
    BounceSlash,
    BounceBackslash,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '|' => Tile::SplitVertical,
            '-' => Tile::SplitHorizontal,
            '/' => Tile::BounceSlash,
            '\\' => Tile::BounceBackslash,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::SplitVertical => write!(f, "|"),
            Tile::SplitHorizontal => write!(f, "-"),
            Tile::BounceSlash => write!(f, "/"),
            Tile::BounceBackslash => write!(f, "\\"),
        }
    }
}

// fn draw_map(map: &Vec<Vec<Tile>>, current: Coordinate, dir: Direction, energized_tiles: &HashSet<Coordinate>) {
//     for (y, row) in map.iter().enumerate() {
//         for (x, tile) in row.iter().enumerate() {
//             let (es, ee) = if energized_tiles.contains(&(x as isize, y as isize)) {
//                 ("\x1b[93m", "\x1b[0m")
//             }
//             else {
//                 ("", "")
//             };
//             if (x as isize, y as isize) == current {
//                 eprint!("{es}{dir}{ee}");
//             }
//             else {
//                 eprint!("{es}{tile}{ee}");
//             }
//         }
//         eprintln!();
//     }
//     eprintln!();
// }

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            _ => false,
        }
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

type Coordinate = (isize, isize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Action {
    Continue,
    Stop,
    Bounce(Direction),
    SplitVertical,
    SplitHorizontal,
}

fn get_action(map: &Vec<Vec<Tile>>, coord: Coordinate, dir: Direction) -> Action {
    // fall off the edge of the map
    if coord.0 < 0
        || coord.1 < 0
        || coord.0 >= map[0].len() as isize
        || coord.1 >= map.len() as isize
    {
        return Action::Stop;
    }

    match map[coord.1 as usize][coord.0 as usize] {
        Tile::SplitVertical if dir.is_horizontal() => Action::SplitVertical,
        Tile::SplitHorizontal if dir.is_vertical() => Action::SplitHorizontal,
        Tile::BounceSlash => match dir {
            Direction::Right => Action::Bounce(Direction::Up),
            Direction::Up => Action::Bounce(Direction::Right),
            Direction::Down => Action::Bounce(Direction::Left),
            Direction::Left => Action::Bounce(Direction::Down),
        },
        Tile::BounceBackslash => match dir {
            Direction::Up => Action::Bounce(Direction::Left),
            Direction::Down => Action::Bounce(Direction::Right),
            Direction::Left => Action::Bounce(Direction::Up),
            Direction::Right => Action::Bounce(Direction::Down),
        },
        _ => Action::Continue,
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(Tile::from).collect())
        .collect()
}

fn count_energized_tiles(map: &Vec<Vec<Tile>>, ray: (Coordinate, Direction)) -> usize {
    let num_tiles = map.len() * map[0].len();
    let mut evaluated_rays: HashSet<(Coordinate, Direction)> = HashSet::new();
    let mut pending_rays: Vec<(Coordinate, Direction)> = Vec::new();
    let mut energized_tiles: HashSet<Coordinate> = HashSet::new();

    pending_rays.push(ray);
    'rays: while !pending_rays.is_empty() {
        let ray = pending_rays.pop().expect("pending_rays is not empty");
        //eprintln!("evaluating ray: {ray:?}");
        evaluated_rays.insert(ray);
        let (start, current_dir) = ray;

        let mut current = start;
        if current.0 >= 0
            && current.1 < map[0].len() as isize
            && current.1 >= 0
            && current.1 < map.len() as isize
        {
            energized_tiles.insert(current);
        }
        // draw_map(&map, current, current_dir, &energized_tiles);
        'raymarch: loop {
            if energized_tiles.len() == num_tiles {
                break 'rays;
            }

            current = (
                current.0 + current_dir.delta().0,
                current.1 + current_dir.delta().1,
            );
            energized_tiles.insert(current);

            // draw_map(&map, current, current_dir, &energized_tiles);

            match get_action(&map, current, current_dir) {
                Action::Continue => continue 'raymarch,
                Action::Stop => {
                    //eprintln!("killing ray");
                    // it took me way too long to realize that this was the problem
                    energized_tiles.remove(&current);
                    break 'raymarch;
                }
                Action::Bounce(new_dir) => {
                    //eprintln!("bouncing to {new_dir}");
                    if !evaluated_rays.contains(&(current, new_dir)) {
                        pending_rays.push((current, new_dir));
                    }
                    break 'raymarch;
                }
                Action::SplitVertical => {
                    //eprintln!("splitting vertical");
                    if !evaluated_rays.contains(&(current, Direction::Up)) {
                        pending_rays.push((current, Direction::Up));
                    }
                    if !evaluated_rays.contains(&(current, Direction::Down)) {
                        pending_rays.push((current, Direction::Down));
                    }
                    break 'raymarch;
                }
                Action::SplitHorizontal => {
                    //eprintln!("splitting horizontal");
                    if !evaluated_rays.contains(&(current, Direction::Left)) {
                        pending_rays.push((current, Direction::Left));
                    }
                    if !evaluated_rays.contains(&(current, Direction::Right)) {
                        pending_rays.push((current, Direction::Right));
                    }
                    break 'raymarch;
                }
            }
        }
    }

    // for y in 0..map.len() {
    //     for x in 0..map[0].len() {
    //         if energized_tiles.contains(&(x as isize, y as isize)) {
    //             //eprint!("\x1b[91m#\x1b[0m");
    //         }
    //         else {
    //             //eprint!(".");
    //         }
    //     }
    //     //eprintln!();
    // }
    // //eprintln!();

    // let mut derp = energized_tiles.iter().collect::<Vec<_>>();
    // derp.sort();
    // for (i, coord) in derp.iter().enumerate() {
    //     //eprintln!("{i: >2}: {coord:?}");
    // }

    // draw_map(&map, (0, 0), Direction::Right, &energized_tiles);
    energized_tiles.len()
}

pub fn part1(input: &str) -> usize {
    let map = parse(input.trim());
    count_energized_tiles(&map, ((-1, 0), Direction::Right))
}

pub fn part2(input: &str) -> usize {
    let map = parse(input.trim());
    let (width, height) = (map[0].len() as isize, map.len() as isize);
    let mut rays: Vec<(Coordinate, Direction)> = Vec::new();
    for y in 0..height {
        rays.push(((-1, y), Direction::Right));
        rays.push(((width, y), Direction::Left));
    }
    for x in 1..width {
        rays.push(((x, -1), Direction::Down));
        rays.push(((x, height), Direction::Up));
    }

    rays.into_par_iter()
        .map(|ray| count_energized_tiles(&map, ray))
        .max()
        .unwrap_or(0)
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn day15_sample_part1() {
        assert_eq!(part1(SAMPLE), 46);
    }

    #[test]
    fn day15_sample_part2() {
        assert_eq!(part2(SAMPLE), 51);
    }
}
