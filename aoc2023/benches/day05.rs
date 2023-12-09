use aoc2023::day05::part1;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn from_elem(c: &mut Criterion) {
    let path = std::path::PathBuf::from("inputs").join("day05.txt");
    let input = std::fs::read_to_string(&path).expect("can read input");
    let input = input.as_str();

    c.bench_with_input(
        BenchmarkId::new("Part 1", path.display()),
        &input,
        |b, &s| {
            b.iter(|| part1(s));
        },
    );
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
