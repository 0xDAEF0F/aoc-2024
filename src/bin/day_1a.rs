use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("data/day_1.txt")?;
    let reader = BufReader::new(file);

    let mut vec_one: Vec<u64> = vec![];
    let mut vec_two: Vec<u64> = vec![];

    for line in reader.lines() {
        let line = line?;

        let mut iter = line.split_whitespace();

        let fst: u64 = iter.next().expect("first word error").parse()?;
        let snd: u64 = iter.next().expect("second word error").parse()?;

        vec_one.push(fst);
        vec_two.push(snd);
    }

    vec_one.sort();
    vec_two.sort();

    let mut difference: u64 = 0;
    for (a, b) in vec_one.into_iter().zip(vec_two) {
        let diff = if a >= b { a - b } else { b - a };

        difference += diff;
    }

    println!("difference: {}", difference);

    Ok(())
}
