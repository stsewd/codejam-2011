use std::io::{self, BufRead};

fn solve(x: i32, y: i32, z: i32) -> i32 {
    let mut n = x;
    let mut sum = 0;
    for i in 1..z + 1 {
        if i >= y {
            sum += n;
        }
        n *= i * 2;
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();
    let z = iter.next().unwrap();
    println!("{}", solve(x, y, z));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(33, solve(3, 1, 3));
        assert_eq!(464, solve(8, 2, 4));
        assert_eq!(10, solve(5, 2, 2));
    }
}

