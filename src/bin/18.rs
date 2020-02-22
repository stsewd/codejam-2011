use std::io::{self, BufRead};

fn solve(array: &mut Vec<&str>) {
    quicksort(array, 0, (array.len() - 1) as i32);
}

fn quicksort(array: &mut Vec<&str>, lo: i32, hi: i32) {
    if lo >= hi {
        return;
    }
    let p = partition(array, lo, hi);
    quicksort(array, lo, p);
    quicksort(array, p + 1, hi);
}

fn partition(array: &mut Vec<&str>, lo: i32, hi: i32) -> i32 {
    let mut i = lo;
    let mut j = hi;
    let pivot_index = lo + ((hi - lo) / 2);
    let pivot = array[pivot_index as usize];

    loop {
        while array[i as usize] < pivot {
            i += 1;
        }

        while array[j as usize] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }
        array.swap(i as usize, j as usize);
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut array: Vec<&str> = line.split_whitespace().collect();
    solve(&mut array);
    println!("{}", array.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut array = vec!["test", "contest", "programming", "more"];
        solve(&mut array);
        assert_eq!(vec!["contest", "more", "programming", "test"], array);
    }
}
