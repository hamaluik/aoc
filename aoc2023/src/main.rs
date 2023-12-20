use anyhow::{Context, Result};
use aoc2023::*;
use rayon::prelude::*;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod inputs;

#[derive(Copy, Clone)]
enum Status {
    Done,
    Pending,
    Future,
}

impl Status {
    fn color(&self) -> Color {
        match self {
            Status::Done => Color::Green,
            Status::Pending => Color::Yellow,
            Status::Future => Color::White,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Done => "✓".to_string(),
            Status::Pending => "…".to_string(),
            Status::Future => " ".to_string(),
        }
    }
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    inputs::load_inputs().with_context(|| "Failed to load inputs")?;

    let days: Vec<usize> = if let Some(day) = std::env::args().skip(1).next() {
        vec![day
            .parse()
            .with_context(|| format!("Failed to parse day: {}", day))?]
    } else {
        (1..=25).collect()
    };

    let results: Vec<((Option<usize>, Option<usize>), (Status, Status), f64)> = days
        .par_iter()
        .map(|day| {
            let input =
                std::fs::read_to_string(PathBuf::from("inputs").join(format!("day{:02}.txt", day)))
                    .ok();
            let has_input = input.is_some();
            let now = std::time::Instant::now();
            let res = match (day, input) {
                (1, Some(input)) => (day01::run(&input), true),
                (2, Some(input)) => (day02::run(&input), true),
                (3, Some(input)) => (day03::run(&input), true),
                (4, Some(input)) => (day04::run(&input), true),
                (5, Some(input)) => (day05::run(&input), true),
                (6, Some(input)) => (day06::run(&input), true),
                (7, Some(input)) => (day07::run(&input), true),
                (8, Some(input)) => (day08::run(&input), true),
                (9, Some(input)) => (day09::run(&input), true),
                (10, Some(input)) => (day10::run(&input), true),
                (11, Some(input)) => (day11::run(&input), true),
                (14, Some(input)) => (day14::run(&input), true),
                (15, Some(input)) => (day15::run(&input), true),
                (16, Some(input)) => (day16::run(&input), true),
                (17, Some(input)) => (day17::run(&input), true),
                (18, Some(input)) => (day18::run(&input), true),
                (19, Some(input)) => (day19::run(&input), true),
                _ => ((None, None), has_input),
            };
            (res, now.elapsed().as_secs_f64())
        })
        .map(|day| match day {
            (((p1, p2), true), elapsed) => (
                (p1, p2),
                match (p1.is_some(), p2.is_some()) {
                    (true, true) => (Status::Done, Status::Done),
                    (true, false) => (Status::Done, Status::Pending),
                    (false, true) => (Status::Pending, Status::Pending),
                    (false, false) => (Status::Pending, Status::Pending),
                },
                elapsed,
            ),
            (((p1, p2), false), elapsed) => ((p1, p2), (Status::Future, Status::Future), elapsed),
        })
        .collect();

    let results: Vec<((String, Status), (String, Status), String)> = results
        .into_iter()
        .map(|((p1, p2), (s1, s2), elapsed)| {
            let p1 = p1
                .map(|p1| p1.to_string())
                .unwrap_or_else(|| s1.to_string());
            let p2 = p2
                .map(|p2| p2.to_string())
                .unwrap_or_else(|| s2.to_string());
            ((p1, s1), (p2, s2), format!("{:.6}s", elapsed))
        })
        .collect();

    let (p1_width, p2_width) =
        results
            .iter()
            .fold((0, 0), |(p1_width, p2_width), ((p1, _), (p2, _), _)| {
                (p1_width.max(p1.len()), p2_width.max(p2.len()))
            });
    let p1_width = p1_width.max("Part 1".len());
    let p2_width = p2_width.max("Part 2".len());

    println!(
        "╒═════╤═{p1:═<width1$}═╤═{p2:═<width2$}═╤═══════════╕",
        p1 = "",
        p2 = "",
        width1 = p1_width,
        width2 = p2_width
    );
    println!(
        "│ Day │ {p1: <width1$}Part 1 │ {p2: <width2$}Part 2 │   Elapsed │",
        p1 = "",
        p2 = "",
        width1 = (p1_width - "Part 1".len()),
        width2 = (p2_width - "Part 2".len())
    );
    println!(
        "├─────┼─{p1:─<width1$}─┼─{p2:─<width2$}─┼───────────┤",
        p1 = "",
        p2 = "",
        width1 = p1_width,
        width2 = p2_width
    );
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for (day, ((p1, s1), (p2, s2), elapsed)) in days.into_iter().zip(results.into_iter()) {
        write!(stdout, "│ {day:>3} │ ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(s1.color())))?;
        write!(stdout, "{p1:>width1$}", p1 = p1, width1 = p1_width,)?;
        stdout.reset()?;
        write!(stdout, " │ ",)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(s2.color())))?;
        write!(stdout, "{p2:>width2$}", p2 = p2, width2 = p2_width,)?;
        write!(stdout, " │ ",)?;
        if elapsed != "0.000000s" {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
            write!(stdout, "{elapsed}")?;
        } else {
            stdout.set_color(ColorSpec::new().set_fg(None))?;
            write!(stdout, "         ")?;
        }
        stdout.reset()?;
        writeln!(stdout, " │")?;
    }
    println!(
        "╘═════╧═{p1:═<width1$}═╧═{p2:═<width2$}═╧═══════════╛",
        p1 = "",
        p2 = "",
        width1 = p1_width,
        width2 = p2_width
    );

    Ok(())
}
