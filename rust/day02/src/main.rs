use std::fs;

struct Run {
    id: i32,
    games: Vec<Game>,
}
struct Game {
    green: i32,
    blue: i32,
    red: i32,
}

fn main() {
    let file_path = "puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", solution2(&contents))
}
fn solution(input: &str) -> i32 {
    let games = input.lines().map(parse_run);
    return games
        .filter(|run| run.games.iter().all(|game| is_valid_game(game)))
        .map(|x| x.id)
        .sum();
}

fn solution2(input: &str) -> i32 {
    let games = input.lines().map(parse_run);
    return games
        .map(|run| {
            run.games.iter().fold(
                Game {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |res, game| Game {
                    red: std::cmp::max(res.red, game.red),
                    green: std::cmp::max(res.green, game.green),
                    blue: std::cmp::max(res.blue, game.blue),
                },
            )
        })
        .map(|x| x.red * x.blue * x.green)
        .sum();
}
fn parse_run(line: &str) -> Run {
    let res = line.split(":").flat_map(|f| f.split(";"));
    let mut game: Option<i32> = None;
    let mut games: Vec<Game> = Vec::new();
    for a in res {
        if a.starts_with("Game") {
            game = Some(get_number(a))
        } else {
            games.push(parse(a))
        }
    }
    return Run {
        id: game.expect("Need a game id"),
        games: games,
    };
}
fn parse(game: &str) -> Game {
    let res = game.split(",").map(|x| x.trim());
    let mut blue: i32 = 0;
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    for a in res {
        if a.ends_with("blue") {
            blue = (get_number(a))
        }
        if a.ends_with("red") {
            red = (get_number(a))
        }
        if a.ends_with("green") {
            green = (get_number(a))
        }
    }
    return Game {
        green: green,
        blue: blue,
        red: red,
    };
}
fn get_number(line: &str) -> i32 {
    let idx = line.find(" ").expect("There should be a space");
    let slice = if line.starts_with("Game") {
        line.get(line.char_indices().nth(idx + 1).unwrap().0..)
    } else {
        line.get(0..line.char_indices().nth(idx).unwrap().0)
    };
    println!("{}", line);
    dbg!(slice);
    return slice.expect("should be an int").parse().unwrap();
}

fn is_valid_game(game: &Game) -> bool {
    let max = Game {
        red: 12,
        blue: 14,
        green: 13,
    };

    return game.red <= max.red && game.blue <= max.blue && game.green <= max.green;
}
mod tests {
    use super::*;

    #[test]
    fn expectSimple() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = solution(input);
        assert_eq!(8, res);
    }

    #[test]
    fn expectSimpleLine() {
        let input = "Game 10: 100 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let res = solution(input);
        assert_eq!(0, res);
    }
    #[test]
    fn expectSimpleSol2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = solution2(input);
        assert_eq!(2286, res);
    }
}
