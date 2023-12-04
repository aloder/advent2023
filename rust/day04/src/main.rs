use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let res = solution(&contents);
    println!("Solution 1: {}", res);
    println!("Solution 2: {}", solution2(&contents));
}

#[derive(Debug)]
struct Game {
    card_number: i32,
    match_numbers: HashSet<i32>,
    numbers: Vec<i32>,
    num_matches: i32,
    points: i32,
}

#[derive(Debug)]
struct Copies {
    card_number_copies: HashMap<i32, i32>,
    number_of_cards: i32,
}

fn solution(input: &str) -> i32 {
    input.lines().map(|line| parse_game(line).points).sum()
}

fn solution2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse_game(line))
        .fold(
            Copies {
                card_number_copies: HashMap::new(),
                number_of_cards: 0,
            },
            |mut acc, game| {
                let num = *acc.card_number_copies.get(&game.card_number).unwrap_or(&1);
                acc.number_of_cards += num;
                for i in game.card_number..(game.card_number + game.num_matches + 1) {
                    *acc.card_number_copies.entry(i).or_insert(1) += num;
                }
                acc
            },
        )
        .number_of_cards
}

fn parse_game(input: &str) -> Game {
    let mut game = Game {
        card_number: parse_card_number(input),
        match_numbers: parse_match_numbers(input),
        numbers: parse_numbers(input),
        num_matches: 0,
        points: 0,
    };
    score_game(&mut game);
    game
}
fn score_game(game: &mut Game) {
    let mut points = 0;
    let mut num_matches = 0;
    for number in game.numbers.iter() {
        if game.match_numbers.contains(number) {
            num_matches += 1;
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }
    game.points = points;
    game.num_matches = num_matches;
}
fn parse_match_numbers(input: &str) -> HashSet<i32> {
    input
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split("|")
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split("|")
        .last()
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn parse_card_number(input: &str) -> i32 {
    input
        .split(":")
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solution;
    use super::solution2;

    #[test]
    fn expect_sol1_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = solution(input);
        assert_eq!(13, res);
    }
    #[test]
    fn expect_sol2_example() {
        let input = "Card   1: 87 75 80 68 71 57 58 59 70 48 | 56 67 75 76 31 49 48 22 43 68 98 86 70 91 27 46  4 87 72 37 71 58 29 79 80";

        let res = solution(input);
        assert_eq!(0, res);
    }
    #[test]
    fn expect_sol2_actual_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = solution2(input);
        assert_eq!(30, res);
    }
}
