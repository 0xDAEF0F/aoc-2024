use std::collections::{HashMap, HashSet};

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../../data/day_19.txt");

    let (unparsed_patterns, unparsed_lines) = input.split_once("\n\n").unwrap();

    let patterns: HashSet<&str> = unparsed_patterns.split(", ").collect();
    let lines: Vec<&str> = unparsed_lines.lines().collect();

    let mut designs_possible = 0;
    let mut memo = HashMap::new();

    for &target_design in lines.iter() {
        designs_possible += num_ways(target_design, &patterns, &mut memo);
    }

    println!("designs_possible: {}", designs_possible);
    println!("time elapsed: {}ms", now.elapsed().as_millis());
}

fn num_ways<'a>(
    target_pattern: &'a str,
    patterns: &HashSet<&'a str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&result) = memo.get(target_pattern) {
        return result;
    }

    let mut total_ways = 0;

    for (prefix, suffix) in split_pairs(target_pattern) {
        if patterns.contains(prefix) {
            let num_ways_suffix = num_ways(suffix, patterns, memo);
            memo.insert(suffix, num_ways_suffix);
            total_ways += num_ways_suffix;
        }
    }

    if patterns.contains(target_pattern) {
        total_ways += 1;
    }

    memo.insert(target_pattern, total_ways);
    total_ways
}

fn split_pairs(s: &str) -> Vec<(&str, &str)> {
    (1..s.len()).map(|i| s.split_at(i)).collect()
}
