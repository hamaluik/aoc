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

fn process(input: &str) -> usize {
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

fn main() {
    let input = std::fs::read_to_string("input").expect("can read file");
    let res = process(&input);
    println!("res => {res}");
}

#[cfg(test)]
mod test {
    fn sample() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(super::process(input), 281);
    }
}
