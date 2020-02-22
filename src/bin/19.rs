use std::io::{self, BufRead};

fn solve(x: i32) -> i32 {
    let mut x = x;
    let mut ones = 0;
    while x > 0 {
        if x % 2 != 0 {
            ones += 1;
        }
        x >>= 1;
    }
    ones
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let x: i32 = line.parse().unwrap();
    println!("{}", solve(x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, solve(0));
        assert_eq!(1, solve(1));
        assert_eq!(1, solve(4));
        assert_eq!(3, solve(25));
    }
}
