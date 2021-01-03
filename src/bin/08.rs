use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve(line: &str) -> u32 {
    let space_chars: HashSet<char> = [' ', ',', '.'].iter().cloned().collect();
    let mut a_count = 0;
    let mut has_a = false;
    for c in line.chars() {
        if space_chars.contains(&c) {
            if has_a {
                a_count += 1;
                has_a = false;
            }
        } else {
            has_a = has_a || c == 'A' || c == 'a';
        }
    }
    if has_a {
        a_count += 1;
    }
    a_count
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
        assert_eq!(solve("this is a sample text, it has a lot of analysis."), 5);
    }
}
