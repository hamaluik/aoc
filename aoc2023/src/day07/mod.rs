mod part1;
mod part2;

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1::part1(input)), Some(part2::part2(input)))
}
