use std::io::{self, BufRead};

fn solve(x: f64, y: f64) -> f64 {
    if x <= 0.0 || y <= 0.0 {
        return -1.0;
    }
    if x > 255.0 || y > 255.0 {
        return -1.0;
    }
    get_number(x, y)
}

fn get_number(x: f64, y: f64) -> f64 {
    x / y
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();

    let x: f64 = iter.next().unwrap().parse().unwrap();
    let y: f64 = iter.next().unwrap().parse().unwrap();
    println!("{}", solve(x, y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie() {
        assert_eq!(solve(60.0, 3.0), 20.0);
    }
}
