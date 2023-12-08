use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let directions = lines.next().expect("directions").chars().map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("invalid direction"),
    });

    let nodes = lines
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let mut parts = line.split(" = ");
            let name = parts.next().expect("node name");
            let nodes = parts
                .next()
                .expect("nodes")
                .trim_matches(|c| c == '(' || c == ')');
            let mut nodes = nodes.splitn(2, ", ");
            let left = nodes.next().expect("left node");
            let right = nodes.next().expect("right node");

            Some((name, (left, right)))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    (directions.collect(), nodes)
}

fn part1(input: &str) -> usize {
    let (directions, nodes) = parse(input);
    let mut current = "AAA";
    let mut i = 0;
    loop {
        let (left, right) = nodes.get(current).expect("current node");
        current = match directions[i % directions.len()] {
            Direction::Left => left,
            Direction::Right => right,
        };
        i += 1;
        if current == "ZZZ" {
            return i;
        }
    }
}

fn lcm<I: IntoIterator<Item = usize>>(iter: I) -> usize {
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    iter.into_iter().fold(1, |a, b| a * b / gcd(a, b))
}

fn part2(input: &str) -> usize {
    let (directions, nodes) = parse(input);
    let nodes = nodes
        .keys()
        .par_bridge()
        .filter_map(|n| if n.ends_with("A") { Some(n) } else { None })
        .map(|mut current| {
            let mut i = 0;
            loop {
                let (left, right) = nodes.get(current).expect("current node");
                current = match directions[i % directions.len()] {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                i += 1;
                if current.ends_with("Z") {
                    return i;
                }
            }
        }).collect::<Vec<usize>>();
    lcm(nodes)
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE1: &'static str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
    const SAMPLE2: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    const SAMPLE3: &'static str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn day08_sample1_part1() {
        assert_eq!(part1(SAMPLE1), 2);
    }

    #[test]
    fn day08_sample2_part1() {
        assert_eq!(part1(SAMPLE2), 6);
    }

    #[test]
    fn day08_sample3_part2() {
        assert_eq!(part2(SAMPLE3), 6);
    }
}
