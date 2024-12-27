use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("data/day_13.txt").unwrap();
    let reader = BufReader::new(file);

    let regex = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    let iter = reader
        .lines()
        .filter_map(|l| l.ok().filter(|s| !s.is_empty()))
        .chunks(3);

    let mut global_result = 0.0;

    for chunk in &iter {
        let (a, c, b, d, n, z) = chunk
            .flat_map(|line| {
                let capture = regex.captures(&line).unwrap();
                vec![
                    capture[1].parse::<f64>().unwrap(),
                    capture[2].parse::<f64>().unwrap(),
                ]
            })
            .collect_tuple()
            .unwrap();
        if let Some((a, b)) = solve_system(a, b, n, c, d, z) {
            if is_integer_with_epsilon(a) && is_integer_with_epsilon(b) {
                global_result += a * 3.0 + b
            }
        }
    }

    println!("global result: {}", global_result);
}

fn solve_system(a: f64, b: f64, n: f64, c: f64, d: f64, z: f64) -> Option<(f64, f64)> {
    // Calculate determinant of coefficient matrix
    let det = a * d - b * c;

    // Check if system has a unique solution
    if det == 0.0 {
        return None;
    }

    // Calculate x using Cramer's Rule
    let det_x = n * d - b * z;
    let x = det_x / det;

    // Calculate y using Cramer's Rule
    let det_y = a * z - n * c;
    let y = det_y / det;

    Some((x, y))
}

fn is_integer_with_epsilon(x: f64) -> bool {
    (x.fract()).abs() < f64::EPSILON
}
