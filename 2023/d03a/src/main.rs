fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit() && !c.is_ascii_whitespace()
}

fn is_adjacent((sx, sy): (usize, usize), (nx1, ny): (usize, usize), xlen: usize) -> bool {
    let sx = sx as isize;
    let sy = sy as isize;
    let nx1 = nx1 as isize;
    let ny = ny as isize;
    let xlen = xlen as isize;
    match ny - sy {
        -1 | 0 | 1 => sx >= (nx1 - 1) && sx <= (nx1 + xlen),
        _ => false,
    }
}

fn run(input: &str) -> usize {
    let symbol_locations = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| if is_symbol(c) { Some(((x, y), c)) } else { None })
        })
        .collect::<Vec<((usize, usize), char)>>();

    let re = regex::Regex::new(r"\d+").unwrap();
    let mut numbers: Vec<((usize, usize), &str)> = Vec::default();
    for (y, line) in input.lines().enumerate() {
        for number in re.find_iter(line) {
            let (x, e) = (number.start(), number.end());
            let number = &line[x..e];
            numbers.push(((x, y), number));
        }
    }

    // if its dumb but it works...
    let mut sum: usize = 0;
    for symbol in symbol_locations.iter() {
        for number in numbers.iter() {
            if is_adjacent(symbol.0, number.0, number.1.len()) {
                sum += number.1.parse::<usize>().unwrap();
            }
        }   
    }

    sum
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
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(run(input), 4361);
    }

    #[test]
    fn solution() {
        let input = std::fs::read_to_string("input").expect("can read file");
        assert_eq!(run(&input), 536202);
    }
}
