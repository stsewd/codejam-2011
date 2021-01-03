use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn solve(array: &[i32]) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = array.get(1..).unwrap().iter().sum();

    for (index, &e) in array.get(1..).unwrap().iter().enumerate() {
        if left == right {
            return i32::try_from(index).unwrap();
        }
        left += array.get(index as usize).unwrap();
        right -= e;
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let array = Vec::from_iter(line.split_whitespace().map(|n| n.parse::<i32>().unwrap()));
    println!("{}", solve(&array));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let array: Vec<i32> = vec![-7, 1, 5, 2, -4, 3, 0];
        assert_eq!(3, solve(&array));

        let array: Vec<i32> = vec![0, 5, 0];
        assert_eq!(1, solve(&array));
    }
}
