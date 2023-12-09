fn roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = b * b - 4.0 * a * c;
    let d_sqrt = d.sqrt();
    let x1 = (-b + d_sqrt) / (2.0 * a);
    let x2 = (-b - d_sqrt) / (2.0 * a);
    (x1.min(x2), x1.max(x2))
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    const ACCELERATION: usize = 1;
    let race_duration = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap());
    let distance_record = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap());

    race_duration.zip(distance_record).map(|(t, record)| {
        // let t_hold_max = t / 2;
        let t_hold_record = roots(
            -(ACCELERATION as f64),
            (ACCELERATION * t) as f64,
            -(record as f64),
        );
        let t_hold_record = (
            t_hold_record.0.ceil() as usize,
            t_hold_record.1.ceil() as usize,
        );

        // let d_max = ACCELERATION * t_hold_max * (t - t_hold_max);
        let d_record = (
            ACCELERATION * t_hold_record.0 * (t - t_hold_record.0),
            ACCELERATION * t_hold_record.1 * (t - t_hold_record.1),
        );

        let mut count = t_hold_record.1 - t_hold_record.0;
        if d_record.0 == d_record.1 {
            count -= 1;
        }
        // eprintln!("t: {}, record: {}", t, record);
        // eprintln!(
        //     "t_hold_max: {}, t_hold_record: {:?}, count: {}",
        //     t_hold_max, t_hold_record, count
        // );
        // eprintln!("d_max: {}, d_record: {:?}", d_max, d_record);
        // eprintln!();
        count
    })
    .fold(1, |acc, count| acc * count)
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    const ACCELERATION: usize = 1;
    let race_duration = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance_record = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let t = race_duration;
    let record = distance_record;

    // let t_hold_max = t / 2;
    let t_hold_record = roots(
        -(ACCELERATION as f64),
        (ACCELERATION * t) as f64,
        -(record as f64),
    );
    let t_hold_record = (
        t_hold_record.0.ceil() as usize,
        t_hold_record.1.ceil() as usize,
    );

    // let d_max = ACCELERATION * t_hold_max * (t - t_hold_max);
    let d_record = (
        ACCELERATION * t_hold_record.0 * (t - t_hold_record.0),
        ACCELERATION * t_hold_record.1 * (t - t_hold_record.1),
    );

    let mut count = t_hold_record.1 - t_hold_record.0;
    if d_record.0 == d_record.1 {
        count -= 1;
    }
    // eprintln!("t: {}, record: {}", t, record);
    // eprintln!(
    //     "t_hold_max: {}, t_hold_record: {:?}, count: {}",
    //     t_hold_max, t_hold_record, count
    // );
    // eprintln!("d_max: {}, d_record: {:?}", d_max, d_record);
    // eprintln!();
    count
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn day06_sample_part1() {
        assert_eq!(part1(SAMPLE), 288);
    }

    #[test]
    fn day06_sample_part2() {
        assert_eq!(part2(SAMPLE), 71503);
    }
}


