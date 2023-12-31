#[derive(Debug)]
struct Map {
    destination: usize,
    source: usize,
    length: usize,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut parts = s.split(" ");
        let destination = parts.next().unwrap().parse::<usize>().unwrap();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        Self {
            destination,
            source,
            length,
        }
    }
}

impl Map {
    fn map(&self, x: usize) -> Option<usize> {
        if x >= self.source && x <= self.source + self.length {
            let res = self.destination + (x - self.source);
            Some(res)
        } else {
            None
        }
    }
}

fn map(maps: &Vec<Map>, x: usize) -> usize {
    let map = maps.iter().find_map(|m| m.map(x));
    match map {
        Some(mapped) => mapped,
        None => x,
    }
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<Map>> = Vec::default();
    let mut current_map: Vec<Map> = Vec::default();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains("map:") {
            maps.push(current_map);
            current_map = Vec::default();
            continue;
        }
        let map = Map::from(line.trim());
        current_map.push(map);
    }
    maps.push(current_map);

    seeds
        .iter()
        .map(|seed| {
            let mut x: usize = *seed;
            for mapping in maps.iter() {
                x = map(mapping, x);
            }
            x
        })
        .min()
        .expect("min value")
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), None)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#;

    #[test]
    fn day05_sample_part1() {
        assert_eq!(part1(SAMPLE), 35);
    }
}
