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

        let num_combinations = line.len() - 1;
        let mut combinations_iter = line.into_iter().combinations(num_combinations);

        let is_safe = combinations_iter.any(|comb| is_safe(comb));

        if is_safe {
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
