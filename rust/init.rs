use std::fs;

fn main() {
    let file_path = "puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let res = solution(&contents);
    println!("Solution 1: {}", res);
    println!("Solution 2: {}", solution2(&contents));
}
fn solution(input: &str) -> i32 {
    0
}

fn solution2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::solution;
    use super::solution2;

    #[test]
    fn expect_sol1_example() {
        let input = "";
        let res = solution(input);
        assert_eq!(0, res);
    }
    #[test]
    fn expect_sol2_example() {
        let input = "";
        let res = solution2(input);
        assert_eq!(0, res);
    }
}
