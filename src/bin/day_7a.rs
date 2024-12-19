use anyhow::Result;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("data/day_7.txt")?;
    let reader = BufReader::new(file);

    let mut count: u128 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut split = line.split(':');

        let target: u128 = split.next().unwrap().trim().parse().unwrap();

        let nums: Vec<u128> = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if compute_results(nums).contains(&target) {
            count += target;
        }
    }

    println!("Count: {count}");

    Ok(())
}

fn compute_results(nums: Vec<u128>) -> Vec<u128> {
    let mut iter = nums.into_iter();

    let mut results = vec![iter.next().expect("no first elem")];

    for num in iter {
        let n_r = results
            .into_iter()
            .flat_map(|e| vec![e + num, e * num])
            .collect_vec();
        results = n_r;
    }

    results
}
