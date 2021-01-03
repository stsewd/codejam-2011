use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve(line1: &str, line2: &str) -> String {
    let mut set = HashSet::new();
    for c in line2.chars() {
        set.insert(c.to_lowercase().next().unwrap());
    }
    line1
        .chars()
        .filter(|&c| !set.contains(&c.to_lowercase().next().unwrap()))
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let line1 = iter.next().unwrap().unwrap();
    let line2 = iter.next().unwrap().unwrap();

    println!("{}", &solve(&line1, &line2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie() {
        assert_eq!(
            solve("DevsuCodeJam is just great!", "I am here! :)"),
            "DvsuCodJsjustgt"
        );
    }
}
