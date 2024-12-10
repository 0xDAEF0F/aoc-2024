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

    let mut count: u64 = 0;

    for num in vec_one {
        let times_appear_vec_two = vec_two.iter().filter(|&&x| x == num).count();
        let result = num * times_appear_vec_two as u64;

        count += result;
    }

    println!("product: {}", count);

    Ok(())
}
