use std::io::{self, BufRead};

fn solve(x: u32, y: u32) -> i32 {
    for i in x..y + 1 {
        if is_perfect(i) {
            return i as i32;
        }
    }
    -1
}

fn is_perfect(x: u32) -> bool {
    let mut count = 0;
    let pow: u32 = (x as f64).powf(0.5) as u32;
    for i in 1..=(pow + 1) {
        if x % i == 0 {
            count += i;
        }
    }
    count == x
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let x: u32 = iter.next().unwrap().parse().unwrap();
    let y: u32 = iter.next().unwrap().parse().unwrap();
    println!("{}", solve(x, y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(5, 7), 6);
        assert_eq!(solve(7, 9), -1);
    }
}
