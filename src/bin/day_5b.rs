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

    let mut sum: u64 = 0;

    for line in reader_b.lines() {
        let line = line?;

        let mut va = line
            .split(",")
            .map(|a| a.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut should_add = false;

        'outer: loop {
            for (a, b) in orders.iter() {
                if let Some(i_of_a) = va.iter().position(|e| e == a) {
                    if let Some(i_of_b) = va.iter().position(|e| e == b) {
                        if i_of_a > i_of_b {
                            should_add = true;
                            va[i_of_a] = *b;
                            va[i_of_b] = *a;
                            continue 'outer;
                        }
                    }
                }
            }
            break;
        }

        if should_add {
            let middle = va[va.len() / 2];
            sum += middle;
        }
    }

    println!("{sum}");

    Ok(())
}
