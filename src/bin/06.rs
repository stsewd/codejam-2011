use std::io::{self, BufRead};
use std::iter::FromIterator;

fn solve(line: &str) -> String {
    let mut array = Vec::from_iter(line.split_whitespace());
    array.reverse();
    array.join(" ")
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", solve(&line));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve("this is a test"), "test a is this");
    }
}
