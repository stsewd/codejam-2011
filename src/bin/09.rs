use std::io::{self, BufRead};

fn solve(x: u32) -> bool {
    for i in 1..(x / 2 + 1 + 1) {
        if i * i == x {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let n: u32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    println!("{}", solve(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(25), true);
        assert_eq!(solve(1), true);
        assert_eq!(solve(16), true);
        assert_eq!(solve(14), false);
    }
}
