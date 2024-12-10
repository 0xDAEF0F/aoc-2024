use anyhow::Result;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("data/day_2.txt")?;
    let reader = BufReader::new(file);

    let mut counter: u64 = 0;

    for line in reader.lines() {
        let line = line?;

        let line = line
            .split_whitespace()
            .map(|a| a.parse::<u64>().unwrap())
            .collect_vec();

        if is_safe(line.clone()) {
            counter += 1;
        }
    }

    println!("safe: {}", counter);

    Ok(())
}

fn is_safe(row: Vec<u64>) -> bool {
    let [fst, snd, ..] = &row[..] else { panic!() };

    let is_increasing;

    if snd > fst && snd - fst <= 3 {
        is_increasing = true;
    } else if snd < fst && fst - snd <= 3 {
        is_increasing = false;
    } else {
        return false;
    }

    let mut iter = row.iter().tuple_windows();
    iter.next(); // skip first since we are doing it above

    for (prev, curr) in iter {
        if is_increasing {
            if curr > prev && (curr - prev) <= 3 {
                continue;
            } else {
                return false;
            }
        } else {
            if curr < prev && (prev - curr) <= 3 {
                continue;
            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_safe;
    use itertools::Itertools;

    #[test]
    fn check_is_safe() {
        let input: Vec<Vec<u64>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let results = input.into_iter().map(|line| is_safe(line)).collect_vec();

        assert_eq!(
            (true, false, false, false, false, true),
            (results[0], results[1], results[2], results[3], results[4], results[5])
        );
    }
}
