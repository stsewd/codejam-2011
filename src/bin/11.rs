use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn solve(array: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &i in array {
        let n = map.entry(i).or_insert(0);
        *n += 1;
    }
    let mut sorted_array = Vec::from_iter(map.iter());
    sorted_array.sort_by_key(|&k| (k.1, k.0));
    *sorted_array.last().unwrap().0
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
        let array: Vec<i32> = vec![1, 5, 3, -2, 4, 2, 4, -2, 5, 5, 2, 1, 3];
        assert_eq!(5, solve(&array));
    }
}
