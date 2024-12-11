use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("data/day_3.txt")?;
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;

    let mut count = 0;

    for r#match in regex.find_iter(contents.as_str()) {
        let subs = r#match.as_str().strip_prefix("mul(").unwrap();
        let subs = subs.strip_suffix(")").unwrap();
        let (a, b) = subs
            .split(',')
            .map(|spl| spl.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        count += a * b
    }

    println!("{count}");

    Ok(())
}
