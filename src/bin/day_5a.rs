use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file_a = File::open("data/day_5_ordering.txt")?;
    let file_b = File::open("data/day_5_actions.txt")?;

    let reader_a = BufReader::new(file_a);
    let reader_b = BufReader::new(file_b);

    let mut orders: Vec<(u64, u64)> = vec![];

    for line in reader_a.lines() {
        let line = line?;
        let mut sp = line.split("|").into_iter();
        let a = sp.next().context("no elem")?.parse::<u64>()?;
        let b = sp.next().context("no elem")?.parse::<u64>()?;

        orders.push((a, b));
    }

    let mut sum = 0;

    'outer: for line in reader_b.lines() {
        let line = line?;
        let va = line
            .split(",")
            .map(|a| a.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        for (a, b) in orders.iter() {
            if let Some(i_of_a) = va.iter().position(|e| e == a) {
                if let Some(i_of_b) = va.iter().position(|e| e == b) {
                    // a should come before b
                    if i_of_a > i_of_b {
                        continue 'outer;
                    }
                }
            }
        }

        let mid_elem = va[va.len() / 2];
        sum += mid_elem;
    }

    println!("{sum}");

    Ok(())
}
