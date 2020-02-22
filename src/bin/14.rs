use std::io::{self, BufRead};

fn solve(x: i32, y: i32, z: i32) -> i32 {
    let n = (z / 2) + (z % 2) - 1;
    let x = x + n;
    let y = y + n;
    if z % 2 == 0 {
        return format!("{}{}", y, x).parse().unwrap();
    } else {
        return format!("{}{}", x, y).parse().unwrap();
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut iter = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
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
        assert_eq!(64, solve(5, 3, 3));
        assert_eq!(46, solve(5, 3, 4));
        assert_eq!(512, solve(10, 3, 6));
        assert_eq!(136, solve(10, 3, 7));
    }
}

