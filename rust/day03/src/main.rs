use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let res = solution(&contents);
    println!("Solution 1: {}", res);
    println!("Solution 2: {}", solution2(&contents));
}
fn solution(input: &str) -> i32 {
    let map = create_map(input);
    let contious_numbers = find_contious_numbers(map.clone());
    let valid_cords = valid_cords(map);
    println!("{:?}", valid_cords);
    println!("{:?}", contious_numbers);
    contious_numbers
        .iter()
        .filter(|f| f.cords.iter().any(|cord| valid_cords.contains(cord)))
        .map(|f| f.value)
        .sum()
}

fn solution2(input: &str) -> i32 {
    let map = create_map(input);
    let contious_numbers = find_contious_numbers(map.clone());
    let hash_map_contious_numbers = contious_numbers.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<Cord, ContiousNumber>, f| {
            f.cords.iter().for_each(|cord| {
                acc.insert(cord.clone(), f.clone());
            });
            acc
        },
    );
    let binding = get_valid_symbols(map);
    let valid_symbols: Vec<_> = binding.iter().filter(|x| x.value == '*').collect();
    let ret: i32 = valid_symbols
        .iter()
        .map(|x| {
            cords_around(x.cord.clone())
                .iter()
                .map(|x| hash_map_contious_numbers.get(x))
                .filter(|x| x.is_some())
                .map(|a| a.unwrap())
                .fold(Vec::new(), |mut acc, f| {
                    if !acc.contains(f) {
                        acc.push(f.clone());
                    }
                    acc
                })
        })
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().fold(1, |acc, x| acc * x.value))
        .sum();
    return ret;
}
fn create_map(input: &str) -> Map {
    let mut places = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(Place {
                cord: Cord {
                    x: x as i32,
                    y: y as i32,
                },
                value: c,
            });
        }
        places.push(row);
    }
    Map { places }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct ContiousNumber {
    value: i32,
    cords: Vec<Cord>,
}

fn find_contious_numbers(map: Map) -> Vec<ContiousNumber> {
    let mut contious_numbers = Vec::new();
    for line in map.places.iter() {
        let mut current_contious_number = Vec::new();
        for place in line.iter() {
            if place.value.is_digit(10) {
                current_contious_number.push(place);
            } else {
                if current_contious_number.len() > 0 {
                    contious_numbers.push(ContiousNumber {
                        value: current_contious_number
                            .iter()
                            .fold(String::new(), |acc, f| format!("{}{}", acc, f.value))
                            .parse()
                            .unwrap(),
                        cords: current_contious_number
                            .iter()
                            .map(|f| f.cord.clone())
                            .collect(),
                    });
                    current_contious_number = Vec::new();
                }
            }
        }
        if current_contious_number.len() > 0 {
            contious_numbers.push(ContiousNumber {
                value: current_contious_number
                    .iter()
                    .fold(String::new(), |acc, f| format!("{}{}", acc, f.value))
                    .parse()
                    .unwrap(),
                cords: current_contious_number
                    .iter()
                    .map(|f| f.cord.clone())
                    .collect(),
            });
        }
    }
    contious_numbers
}

#[derive(Debug, Clone)]
struct Map {
    places: Vec<Vec<Place>>,
}

#[derive(Debug, Clone)]
struct Place {
    cord: Cord,
    value: char,
}
struct Symbol {
    value: char,
    cord: Cord,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Cord {
    x: i32,
    y: i32,
}
fn valid_cords(map: Map) -> HashSet<Cord> {
    let mut cords = HashSet::new();
    for place in map.places.iter().flatten() {
        if place.value != '.' && !place.value.is_digit(10) {
            cords_around(place.cord.clone()).iter().for_each(|cord| {
                cords.insert(cord.clone());
            });
        }
    }
    cords
}
fn get_valid_symbols(map: Map) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    for place in map.places.iter().flatten() {
        if place.value != '.' && !place.value.is_digit(10) {
            symbols.push(Symbol {
                value: place.value,
                cord: place.cord.clone(),
            });
        }
    }
    symbols
}
fn cords_around(cord: Cord) -> Vec<Cord> {
    let mut cords = Vec::new();
    cords.push(Cord {
        x: cord.x - 1,
        y: cord.y - 1,
    });
    cords.push(Cord {
        x: cord.x,
        y: cord.y - 1,
    });
    cords.push(Cord {
        x: cord.x + 1,
        y: cord.y - 1,
    });
    cords.push(Cord {
        x: cord.x - 1,
        y: cord.y,
    });
    cords.push(Cord {
        x: cord.x + 1,
        y: cord.y,
    });
    cords.push(Cord {
        x: cord.x - 1,
        y: cord.y + 1,
    });
    cords.push(Cord {
        x: cord.x,
        y: cord.y + 1,
    });
    cords.push(Cord {
        x: cord.x + 1,
        y: cord.y + 1,
    });
    cords
}

#[cfg(test)]
mod tests {
    use super::solution;
    use super::solution2;

    #[test]
    fn expect_sol1_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = solution(input);
        assert_eq!(4361, res);
    }
    #[test]
    fn expect_sol2_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = solution2(input);
        assert_eq!(467835, res);
    }
}
