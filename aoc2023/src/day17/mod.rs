use crate::grid::{Grid, ParseGridChar};

struct Parse;
impl ParseGridChar<u8> for Parse {
    fn parse_grid_char(c: char) -> u8 {
        (c as u8) - ('0' as u8)
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::parse::<Parse>(input);

    let mut dist: Grid<usize> = Grid::construct(grid.width(), grid.height(), usize::MAX);
    let mut prev: Grid<Option<(usize, usize)>> = Grid::construct(grid.width(), grid.height(), None);
    let mut q: Vec<(usize, usize)> = grid.iter_coords_row_major().collect();
    dist.set(0, 0, 0);

    'djikstra: while !q.is_empty() {
        let (u, _) = q
            .iter()
            .enumerate()
            .min_by_key(|(_, (x, y))| dist.get(*x, *y))
            .expect("q has elements");
        let (x, y) = q.remove(u);
        if x == grid.width() - 1 && y == grid.height() - 1 {
            break 'djikstra;
        }

        let neighbours = grid
            .neighbour_coords_cardinal(x, y)
            .into_iter()
            .filter(|n| q.contains(n))
            .filter(|n| {
                // can't go in a straight line for more than 3 steps
                let mut s = Vec::new();
                s.push((x, y));
                s.push(*n);
                let p1 = prev.get(x, y);
                if let Some(p1) = p1 {
                    let p2 = prev.get(p1.0, p1.1);
                    s.push(p1);
                    if let Some(p2) = p2 {
                        s.push(p2);

                        // check if all 4 are in a straight line
                        // if so, skip this neighbour
                        if s.iter().all(|(x, _)| *x == s[0].0)
                            || s.iter().all(|(_, y)| *y == s[0].1)
                        {
                            return false;
                        }
                    }
                }
                return true;
            }).collect::<Vec<_>>();
        for v in neighbours.into_iter() {
            let cost = grid.get(v.0, v.1) as usize;
            let alt = dist.get(x, y) + cost;
            if alt < dist.get(v.0, v.1) {
                dist.set(v.0, v.1, alt);
                prev.set(v.0, v.1, Some((x, y)));
            }
        }
    }

    let mut s = Vec::new();
    s.push((grid.width() - 1, grid.height() - 1));
    let mut u = (grid.width() - 1, grid.height() - 1);
    while let Some((x, y)) = prev.get(u.0, u.1) {
        if x == 0 && y == 0 {
            break;
        }
        s.push((x, y));
        u = (x, y);
    }

    let mut draw_grid = Grid::construct(grid.width(), grid.height(), ' ');
    for (x, y) in grid.iter_coords_row_major() {
        let v = grid.get(x, y);
        draw_grid.set(x, y, (v + '0' as u8) as char);
    }
    for (s, l) in s.iter().skip(1).zip(s.iter()) {
        let dir = if s.0 > l.0 {
            '←'
        }
        else if s.0 < l.0 {
            '→'
        }
        else if s.1 > l.1 {
            '↑'
        }
        else if s.1 < l.1 {
            '↓'
        }
        else {
            unreachable!()
        };
        draw_grid.set(s.0, s.1, dir);
    }
    for y in 0..draw_grid.height() {
        for x in 0..draw_grid.width() {
            let (es, ee) = if s.contains(&(x, y)) {
                ("\x1b[93m", "\x1b[0m")
            }
            else {
                ("", "")
            };
            print!("{es}{}{ee}", draw_grid.get(x, y));
        }
        println!();
    }

    s.into_iter()
        .map(|(x, y)| grid.get(x, y) as usize)
        .sum()
}

pub fn part2(_input: &str) -> usize {
    0
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), None)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn day17_sample_part1() {
        assert_eq!(part1(SAMPLE), 102);
    }
}
