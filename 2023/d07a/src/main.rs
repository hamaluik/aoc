fn run(input: &str) -> usize {
    todo!()
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
        let input = r#""#;
        assert_eq!(run(input), todo!());
    }
}
