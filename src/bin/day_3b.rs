use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("data/day_3.txt")?;
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")?;

    let mut count = 0;
    let mut is_on = true;

    for match_ in regex.find_iter(contents.as_str()) {
        let match_ = match_.as_str();
        if match_.starts_with("do") {
            if match_ == "do()" {
                is_on = true;
            } else {
                is_on = false;
            }
            continue;
        }
        let subs = match_.strip_prefix("mul(").unwrap();
        let subs = subs.strip_suffix(")").unwrap();
        let (a, b) = subs
            .split(',')
            .map(|spl| spl.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        if is_on {
            count += a * b;
        }
    }

    println!("{count}");

    Ok(())
}
