use rayon::prelude::*;

fn hash(input: &[u8]) -> usize {
    let mut h = 0;
    for i in input {
        h += *i as usize;
        h *= 17;
        h = h % 256;
    }
    h
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .par_split(',')
        .map(|s| hash(s.as_bytes()))
        .sum()
}

pub fn part2(_input: &str) -> usize {
    0
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), None)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn can_hash() {
        let input = "HASH";
        assert_eq!(hash(input.as_bytes()), 52);
    }

    #[test]
    fn day15_sample_part1() {
        assert_eq!(part1(SAMPLE), 1320);
    }

    #[test]
    #[ignore] // TODO: too early in the morning for me to understand the problem
    fn day15_sample_part2() {
        assert_eq!(part1(SAMPLE), 145);
    }
}
