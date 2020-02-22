use std::collections::HashMap;
use std::io::{self, BufRead};

fn solve(line: &str) -> String {
    let abc: HashMap<char, char> = [
        ('Á', 'a'),
        ('É', 'é'),
        ('Í', 'í'),
        ('Ó', 'o'),
        ('Ú', 'ú'),
        ('Ñ', 'ñ'),
        ('A', 'a'),
        ('B', 'b'),
        ('C', 'c'),
        ('D', 'd'),
        ('E', 'e'),
        ('F', 'f'),
        ('G', 'g'),
        ('H', 'h'),
        ('I', 'i'),
        ('J', 'j'),
        ('K', 'k'),
        ('L', 'l'),
        ('M', 'm'),
        ('N', 'n'),
        ('O', 'o'),
        ('P', 'p'),
        ('Q', 'q'),
        ('R', 'r'),
        ('S', 's'),
        ('T', 't'),
        ('U', 'u'),
        ('V', 'v'),
        ('W', 'w'),
        ('X', 'x'),
        ('Y', 'y'),
        ('Z', 'z'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut solution = String::new();
    for c in line.chars() {
        solution.push(match abc.get(&c) {
            Some(a) => *a,
            None => c,
        });
    }
    solution
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", solve(&line));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve("Ñañito, QUÉ bien! THIS is a sample text, Lorem Ipsum, 2 Be Converted."),
            "ñañito, qué bien! this is a sample text, lorem ipsum, 2 be converted."
        );
    }
}
