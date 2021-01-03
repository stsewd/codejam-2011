use std::io::{self, BufRead};

fn solve(array: &[i32]) -> Vec<i32> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &e in array {
        if e < 0 {
            left.push(e);
        } else {
            right.push(e);
        }
    }
    left.append(&mut right);
    left
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let array: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let solution: Vec<String> = solve(&array).iter().map(|&x| x.to_string()).collect();
    println!("{}", solution.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let array = &[4, -3, -100, 7, 0, 1, -6];
        assert_eq!(vec![-3, -100, -6, 4, 7, 0, 1], solve(array));
    }
}
