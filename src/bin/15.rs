use std::io::{self, BufRead};

fn solve(array: &Vec<i32>) -> i32 {
    let mut max = *array.first().unwrap();
    for i in 0..array.len() {
        let mut current = 0;
        for j in i..array.len() {
            current += array.get(j).unwrap();
            if current > max {
                max = current;
            }
        }
    }
    max
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let array: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", solve(&array));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(14, solve(&vec![4, -3, 7, 2, 4, -5, 1, 2]));
    }
}

