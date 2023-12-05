fn run(input: &str) -> usize {
    let mut lines: Vec<(&str, usize)> = Vec::default();
    for line in input.lines() {
        let mut parts = line.splitn(2, '|');
        let winning = parts.next().expect("winning half");
        let numbers = parts.next().expect("numbers half");

        let winning_numbers = winning.splitn(2, ':').skip(1).next().expect("list of winning numbers");
        let winning_numbers = winning_numbers.trim().split_whitespace().collect::<Vec<&str>>();

        let matching_numbers = numbers.trim().split_whitespace().filter(|n| winning_numbers.contains(n));
        lines.push((line, matching_numbers.count()));
    }

    let mut card_counts = vec![1usize; lines.len()];
    for i in 0..lines.len() {
        for _ in 0..card_counts[i] {
            for j in 0..lines[i].1 {
                card_counts[i + 1 + j] += 1;
            }
        }
    }

    card_counts.into_iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("input").expect("can read file");
    let res = run(&input);
    println!("res => {res}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(run(input), 30);
    }
}
