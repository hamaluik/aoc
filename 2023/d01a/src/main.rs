fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

fn parse_digit(c: char) -> usize {
    c as usize - '0' as usize
}

fn process(input: &str) -> usize {
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

fn main() {
    let input = std::fs::read_to_string("input").expect("can read file");
    let res = process(&input);
    println!("res => {res}");
}

#[cfg(test)]
mod test {
    fn sample() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(super::process(input), 142);
    }
}
