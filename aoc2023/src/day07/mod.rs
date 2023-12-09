mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}
