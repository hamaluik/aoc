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

fn get_label(input: &str) -> &str {
    let end = input.chars().take_while(|c| c != &'=' && c != &'-').count();
    &input[..end]
}

fn box_number(input: &str) -> usize {
    hash(input.as_bytes())
}

pub fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::default(); 256];

    let steps = input.trim().split(',');
    for step in steps {
        let label = get_label(step);
        let box_number = box_number(label);

        // eprintln!("Step {step} -> {label} -> {box_number}");

        let boxx = boxes.get_mut(box_number).expect("box exists");

        if step.contains('-') {
            // remove!
            let idx = boxx.iter().position(|(l, _)| l == &label);
            if let Some(idx) = idx {
                boxx.remove(idx);
                // eprintln!("Removed {} from box {}", label, box_number);
            }
            // else {
            //     eprintln!("Tried to remove {} from box {} but it wasn't there", label, box_number);
            // }
        }
        else if step.contains('=') {
            // add!
            let lens = step.split('=').skip(1).next().expect("lens");
            let len = lens.parse::<usize>().expect("lens number");
            match boxx.iter().position(|(l, _)| l == &label) {
                Some(idx) => {
                    boxx[idx] = (label, len);
                    // eprintln!("Updated {} in box {} to {}", label, box_number, len);
                }
                None => {
                    boxx.push((label, len));
                    // eprintln!("Added {} to box {} with {}", label, box_number, len);
                }
            }
        }

        // eprintln!();
    }

    // for (i, boxx) in boxes.iter().enumerate() {
    //     if !boxx.is_empty() {
    //         eprint!("Box {}: ", i);
    //         for slot in boxx.iter() {
    //             eprint!("[{} {}] ", slot.0, slot.1);
    //         }
    //         eprintln!();
    //     }
    // }
    //
    boxes
        .into_par_iter()
        .enumerate()
        .map(|(i, boxx)| {
            let box_num = i + 1;
            boxx
                .into_iter()
                .enumerate()
                .map(|(j, (_, focal_len))| {
                    let slot_num = j + 1;
                    slot_num * focal_len * box_num
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
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
    fn day15_sample_part2() {
        assert_eq!(part2(SAMPLE), 145);
    }
}
