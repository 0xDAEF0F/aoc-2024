use itertools::Itertools;
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
        if is_design_possible(target_design, &patterns, &mut memo) {
            designs_possible += 1;
        }
    }

    println!("designs_possible: {}", designs_possible);
    println!("time elapsed: {}ms", now.elapsed().as_millis());
}

fn is_design_possible<'a>(
    target_pattern: &'a str,
    patterns: &HashSet<&'a str>,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&result) = memo.get(target_pattern) {
        return result;
    }

    if patterns.contains(target_pattern) {
        memo.insert(target_pattern, true);
        return true;
    }

    let prefixes = (1..=target_pattern.len())
        .rev()
        .map(|i| &target_pattern[..i])
        .collect_vec();

    for prefix in prefixes {
        if patterns.contains(prefix) {
            let suffix = &target_pattern[prefix.len()..];
            if is_design_possible(suffix, patterns, memo) {
                memo.insert(target_pattern, true);
                return true;
            }
        }
    }

    memo.insert(target_pattern, false);
    false
}
