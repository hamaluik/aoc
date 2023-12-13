fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn empty_rows(map: &Vec<Vec<bool>>) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| !c))
        .map(|(i, _)| i)
        .collect()
}

fn empty_columns(map: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut empty = vec![];
    for i in 0..map[0].len() {
        if map.iter().all(|row| !row[i]) {
            empty.push(i);
        }
    }
    empty
}

fn galaxy_locations(map: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut locations = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c {
                locations.push((x, y));
            }
        }
    }
    locations
}

fn galaxy_pairs(galaxies: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs = vec![];
    for (i, &galaxy) in galaxies.iter().enumerate() {
        for &other in &galaxies[i + 1..] {
            pairs.push((galaxy, other));
        }
    }
    pairs
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let empty_rows = empty_rows(&map);
    let empty_columns = empty_columns(&map);
    let galaxies = galaxy_locations(&map);
    let pairs = galaxy_pairs(&galaxies);

    pairs
        .into_iter()
        .map(|(a, b)| {
            let (x1, y1) = a;
            let (x2, y2) = b;

            let mut dx = (x2 as isize - x1 as isize).abs() as usize;
            let mut dy = (y2 as isize - y1 as isize).abs() as usize;

            for xi in x1.min(x2) + 1..x1.max(x2) {
                if empty_columns.contains(&xi) {
                    dx += 1;
                }
            }

            for yi in y1.min(y2) + 1..y1.max(y2) {
                if empty_rows.contains(&yi) {
                    dy += 1;
                }
            }

            dx + dy
        })
        .sum()
}

pub fn part2(input: &str, scale: usize) -> usize {
    let map = parse(input);
    let empty_rows = empty_rows(&map);
    let empty_columns = empty_columns(&map);
    let galaxies = galaxy_locations(&map);
    let pairs = galaxy_pairs(&galaxies);

    // Reduce scale by 1 since we're already intrinsically counting the row/column in dx/dy
    // calculation
    let scale = scale - 1;

    pairs
        .into_iter()
        .map(|(a, b)| {
            let (x1, y1) = a;
            let (x2, y2) = b;

            let mut dx = (x2 as isize - x1 as isize).abs() as usize;
            let mut dy = (y2 as isize - y1 as isize).abs() as usize;

            for xi in x1.min(x2) + 1..x1.max(x2) {
                if empty_columns.contains(&xi) {
                    dx += scale;
                }
            }

            for yi in y1.min(y2) + 1..y1.max(y2) {
                if empty_rows.contains(&yi) {
                    dy += scale;
                }
            }

            dx + dy
        })
        .sum::<usize>()
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input, 1000000)))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn day11_sample_part1() {
        assert_eq!(part1(SAMPLE), 374);
    }

    #[test]
    fn day11_sample_part2() {
        assert_eq!(part2(SAMPLE, 10), 1030);
    }

    #[test]
    fn day11_sample_part3() {
        assert_eq!(part2(SAMPLE, 100), 8410);
    }
}
