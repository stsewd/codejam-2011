use std::io;

fn solve(x: i32, y: i32) -> i32 {
    if x <= 0 || y <= 0 {
        return -1;
    }
    if x > 255 || y > 255 {
        return -1;
    }
    get_number(x) + get_number(y)
}

fn get_number(x: i32) -> i32 {
    if x % 2 == 0 {
        6 - (x - 2)
    } else {
        7 + (x / 2)
    }
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
        assert_eq!(solve(1, 3), 15);
        assert_eq!(solve(8, 9), 11);
    }
}
