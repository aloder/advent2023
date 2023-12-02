use std::char;
use std::fs;

fn main() {
    let file_path = "puzzle2.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");

    let res = split.map(|x| get_left_digit(x.to_string()));
    // for r in res {
    //     println!("{}", r);
    // }
    println!("{}", res.fold(0, |x, j| j + x));
}

fn spells_diget(string: &String, i: usize, is_left: bool) -> Option<char> {
    let pa = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // println!("{string}, {i}, {is_left}");
    let (left, right) = string.split_at(i);
    // println!("{left}, {right}");
    for i in 0..pa.len() {
        if is_left {
            if left.ends_with(pa[i]) {
                return char::from_digit((i + 1).try_into().unwrap(), 10);
            }
        } else {
            if right.starts_with(pa[i]) {
                return char::from_digit((i + 1).try_into().unwrap(), 10);
            }
        }
    }
    return None;
}

fn get_left_digit(line: String) -> i32 {
    let mut j = 0;
    let mut l: Option<char> = None;
    loop {
        if j >= line.len() {
            return 0;
        }
        let c = line.chars().nth(j).expect("Need char");
        j += 1;
        if let Some(a) = spells_diget(&line, j, true) {
            l = Some(a);
            break;
        }
        if is_digit(c) {
            l = Some(c);
            break;
        }
    }
    let mut i = line.len();
    let mut r: Option<char> = None;
    loop {
        if i == 0 || i < j {
            break;
        }
        let c = line.chars().nth(i - 1).expect("Need char");
        i -= 1;
        if let Some(a) = spells_diget(&line, i, false) {
            r = Some(a);
            break;
        }
        if is_digit(c) {
            r = Some(c);
            break;
        }
    }

    if let Some(left_char) = l {
        if let Some(right_char) = r {
            return format!("{}{}", left_char, right_char).parse().unwrap();
        }
        return format!("{}", left_char).parse().unwrap();
    }
    if let Some(right_char) = r {
        return format!("{}", right_char).parse().unwrap();
    }
    return 0;
}

fn is_digit(char: char) -> bool {
    char.is_ascii_digit()
}
