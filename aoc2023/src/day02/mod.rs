struct Game {
    id: usize,
    rounds: Vec<(usize, usize, usize)>,
}

fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

fn extract_and_parse_numbers(s: &str) -> Option<usize> {
    s.chars().filter(is_digit).collect::<String>().parse::<usize>().ok()
}

fn parse_round(round: &str) -> (usize, usize, usize) {
    let parts = round.split(',').collect::<Vec<&str>>();
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for part in parts {
        let mut pull = part.trim().split_whitespace();
        let count = pull.next();
        let colour = pull.next();

        match (count, colour) {
            (Some(count), Some(colour)) if colour.len() > 0 => {
                let count = extract_and_parse_numbers(count);
                if let Some(count) = count {
                    match colour.chars().next().unwrap() {
                        'r' => r += count,
                        'g' => g += count,
                        'b' => b += count,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    (r, g, b)
}

fn parse_game(line: &str) -> Option<Game> {
    // parse the game #
    let mut parts = line.splitn(2, ':');
    let title = parts.next()?;
    let id = extract_and_parse_numbers(title)?;

    let rounds: Vec<_> = parts.next()?.split(';').map(parse_round).collect();

    Some(Game { id, rounds })
}

fn is_game_possible(game: &Game) -> bool {
    game.rounds.iter().all(|g| g.0 <= 12 && g.1 <= 13 && g.2 <= 14)
}

pub fn part1(input: &str) -> usize {
    input.lines().filter_map(parse_game).filter(is_game_possible).map(|g| g.id).sum()
}

fn min_cubes(game: Game) -> (usize, usize, usize) {
    game.rounds.iter().fold((0, 0, 0), |acc, el| (acc.0.max(el.0), acc.1.max(el.1), acc.2.max(el.2)))
}

fn power(game: (usize, usize, usize)) -> usize {
    game.0 * game.1 * game.2
}

pub fn part2(input: &str) -> usize {
    input.lines().filter_map(parse_game).map(min_cubes).map(power).sum()
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), Some(part2(input)))
}

#[cfg(test)]
mod test {
        const SAMPLE: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn day02_sample_part1() {
        assert_eq!(super::part1(SAMPLE), 8);
    }

    #[test]
    fn day02_sample_part2() {
        assert_eq!(super::part2(SAMPLE), 2286);
    }
}

