use anyhow::{Context, Result};
use rayon::prelude::*;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

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
    let days: Vec<usize> = if let Some(day) = std::env::args().skip(1).next() {
        vec![day
            .parse()
            .with_context(|| format!("Failed to parse day: {}", day))?]
    } else {
        (1..=25).collect()
    };

    let results: Vec<((Option<usize>, Option<usize>), (Status, Status))> = days
        .par_iter()
        .map(|day| {
            let input =
                std::fs::read_to_string(PathBuf::from("inputs").join(format!("day{:02}.txt", day)))
                    .ok();
            let has_input = input.is_some();
            match (day, input) {
                (1, Some(input)) => (day01::run(&input), true),
                (2, Some(input)) => (day02::run(&input), true),
                (3, Some(input)) => (day03::run(&input), true),
                (4, Some(input)) => (day04::run(&input), true),
                (5, Some(input)) => (day05::run(&input), true),
                (6, Some(input)) => (day06::run(&input), true),
                (7, Some(input)) => (day07::run(&input), true),
                (8, Some(input)) => (day08::run(&input), true),
                (9, Some(input)) => (day09::run(&input), true),
                _ => ((None, None), has_input),
            }
        })
        .map(|day| match day {
            ((p1, p2), true) => ((p1, p2), match (p1.is_some(), p2.is_some()) {
                (true, true) => (Status::Done, Status::Done),
                (true, false) => (Status::Done, Status::Pending),
                (false, true) => (Status::Pending, Status::Pending),
                (false, false) => (Status::Pending, Status::Pending),
            }),
            ((p1, p2), false) => ((p1, p2), (Status::Future, Status::Future)),
        })
        .collect();

    let results: Vec<((String, Status), (String, Status))> = results
        .into_iter()
        .map(|((p1, p2), (s1, s2))| {
            let p1 = p1
                .map(|p1| p1.to_string())
                .unwrap_or_else(|| s1.to_string());
            let p2 = p2
                .map(|p2| p2.to_string())
                .unwrap_or_else(|| s2.to_string());
            ((p1, s1), (p2, s2))
        })
        .collect();

    let (p1_width, p2_width) = results
        .iter()
        .fold((0, 0), |(p1_width, p2_width), ((p1, _), (p2, _))| {
            (p1_width.max(p1.len()), p2_width.max(p2.len()))
        });
    let p1_width = p1_width.max("Part 1".len());
    let p2_width = p2_width.max("Part 2".len());

    println!(
        "╒═════╤═{p1:═<width1$}═╤═{p2:═<width2$}═╕",
        p1 = "",
        p2 = "",
        width1 = p1_width,
        width2 = p2_width
    );
    println!(
        "│ Day │ {p1: <width1$}Part 1 │ {p2: <width2$}Part 2 │",
        p1 = "",
        p2 = "",
        width1 = (p1_width - "Part 1".len()),
        width2 = (p2_width - "Part 2".len())
    );
    println!(
        "├─────┼─{p1:─<width1$}─┼─{p2:─<width2$}─┤",
        p1 = "",
        p2 = "",
        width1 = p1_width,
        width2 = p2_width
    );
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for (day, ((p1, s1), (p2, s2))) in days.into_iter().zip(results.into_iter()) {
        write!(stdout, "│ {day:>3} │ ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(s1.color())))?;
        write!(stdout, "{p1:>width1$}", p1 = p1, width1 = p1_width,)?;
        stdout.reset()?;
        write!(stdout, " │ ",)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(s2.color())))?;
        write!(stdout, "{p2:>width2$}", p2 = p2, width2 = p2_width,)?;
        stdout.reset()?;
        writeln!(stdout, " │")?;
    }
    println!(
        "╘═════╧═{p1:═<width1$}═╧═{p2:═<width2$}═╛",
        p1 = "",
        p2 = "",
        width1 = p1_width,
        width2 = p2_width
    );

    Ok(())
}
