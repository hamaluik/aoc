use rayon::prelude::*;

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .par_lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| {
                    n.parse::<isize>()
                        .expect(&format!("failed to parse number: {}", n))
                })
                .collect()
        })
        .collect()
}

fn derivative<'i, I: Iterator<Item = &'i isize>>(mut iter: I) -> Vec<isize> {
    let mut prev = iter.next().unwrap();
    iter.map(|n| {
        let diff = n - prev;
        prev = n;
        diff
    })
    .collect()
}

fn derivatives<'i, I: Iterator<Item = &'i isize>>(iter: I) -> Vec<Vec<isize>> {
    let mut derivatives: Vec<Vec<isize>> = Vec::new();
    derivatives.push(derivative(iter));
    loop {
        if derivatives.last().unwrap().iter().all(|n| *n == 0) {
            break;
        }
        derivatives.push(derivative(derivatives.last().unwrap().iter()));
    }
    derivatives
}

fn predict_forward(items: Vec<isize>) -> isize {
    // naive implementation..
    let derivatives = derivatives(items.iter());

    let mut dx = 0;
    for deriv in derivatives.into_iter().rev() {
        dx += deriv.last().unwrap();
    }

    items.last().unwrap() + dx
}

fn predict_backward(items: Vec<isize>) -> isize {
    // naive implementation..
    let derivatives = derivatives(items.iter());

    let mut dx = 0;
    for deriv in derivatives.into_iter().rev() {
        dx = deriv.first().unwrap() - dx;
    }

    items.first().unwrap() - dx
}

fn part1(input: &str) -> isize {
    parse(input).into_par_iter().map(predict_forward).sum()
}

fn part2(input: &str) -> isize {
    parse(input).into_par_iter().map(predict_backward).sum()
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    let p1 = part1(input);
    let p1 = usize::try_from(p1).expect("part1: usize overflow");
    let p2 = part2(input);
    let p2 = usize::try_from(p2).expect("part2: usize overflow");
    (Some(p1), Some(p2))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn day09_sample_part1() {
        assert_eq!(part1(SAMPLE), 114);
    }

    #[test]
    fn day09_sample_part2() {
        assert_eq!(part2(SAMPLE), 2);
    }
}
