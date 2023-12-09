fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

fn parse_digit(c: char) -> usize {
    c as usize - '0' as usize
}

pub fn part1(input: &str) -> usize {
    input.lines().filter_map(|l| {
        let a = l.chars().find(is_digit);
        let b = l.chars().rev().find(is_digit);
        match (a, b) {
            (Some(a), Some(b)) => Some((parse_digit(a) * 10) + parse_digit(b)),
            _ => None,
        }
    })
    .sum()
}

const NUMBERS: [(&'static str, usize); 19] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_number_fwd(line: &str) -> Option<usize> {
    NUMBERS
        .iter()
        .filter_map(|&(word, value)| line.find(word).map(|p| (p, word, value)))
        .min_by_key(|x| x.0)
        .map(|x| x.2)
}

fn find_number_rev(line: &str) -> Option<usize> {
    NUMBERS
        .iter()
        .filter_map(|&(word, value)| line.rfind(word).map(|p| (p, word, value)))
        .max_by_key(|x| x.0)
        .map(|x| x.2)
}

pub fn part2(input: &str) -> usize {
    input.lines().filter_map(|l| {
        let left = find_number_fwd(l);
        let right = find_number_rev(l);
        match (left, right) {
            (Some(left), Some(right)) => Some(left * 10 + right),
            _ => None,
        }
    })
    .sum()
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}

#[cfg(test)]
mod test {
    use super::*;
        const SAMPLE1: &'static str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        const SAMPLE2: &'static str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn day01_sample_part1() {
        assert_eq!(part1(SAMPLE1), 142);
    }

    #[test]
    fn day01_sample_part2() {
        assert_eq!(part2(SAMPLE2), 281);
    }
}

