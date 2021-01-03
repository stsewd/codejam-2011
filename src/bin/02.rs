use std::io;

fn solve(x: i32, y: i32) -> i32 {
    if x <= 0 || y <= 0 {
        return -1;
    }
    if x > 255 || y > 255 {
        return -1;
    }
    get_number(x, y)
}

fn get_number(x: i32, y: i32) -> i32 {
    let mut result = x;
    for i in 1..=y {
        result *= i;
    }
    result
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();

    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", solve(x, y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie() {
        assert_eq!(solve(3, 4), 72);
    }
}
