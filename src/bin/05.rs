use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn solve(array: &[i32]) -> Vec<i32> {
    let mut vect = Vec::from_iter(HashSet::<i32>::from_iter(array.iter().cloned()));
    vect.sort_unstable();
    vect
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut array = Vec::new();
    for item in line.split_whitespace() {
        array.push(item.parse::<i32>().unwrap());
    }

    let mut solution = Vec::new();
    for item in solve(&array) {
        solution.push(item.to_string());
    }
    println!("{}", solution.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie() {
        assert_eq!(
            solve(&[-3, -2, 0, 0, 5, 7, 9, 11, 11, 25]),
            vec![-3, -2, 0, 5, 7, 9, 11, 25]
        );
    }
}
