use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("data/day_10.txt").unwrap();
    let reader = BufReader::new(file);

    let chart = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut trailhead_scores = 0;

    for (i, row) in chart.iter().enumerate() {
        for (j, &square) in row.iter().enumerate() {
            if square == 0 {
                let mut marked_nines = HashSet::<(usize, usize)>::new();
                trailhead_scores += foo(&chart, &mut marked_nines, (i, j), square);
            }
        }
    }

    println!("trailhead_scores: {}", trailhead_scores);
}

fn foo(
    chart: &Vec<Vec<u32>>,
    marked_nines: &mut HashSet<(usize, usize)>,
    curr_coordinates: (usize, usize),
    curr_digit: u32,
) -> u32 {
    if curr_digit == 9 {
        marked_nines.insert(curr_coordinates);
        return 1;
    }

    let mut count = 0;

    let (y, x) = curr_coordinates;

    let top_neighbor = if y > 0 {
        chart.get(y - 1).map(|a| &a[x])
    } else {
        None
    };

    if let Some(&d) = top_neighbor {
        if marked_nines.get(&(y - 1, x)).is_none() && d > curr_digit && (d - curr_digit) == 1 {
            count += foo(chart, marked_nines, (y - 1, x), d);
        }
    }

    let right_neighbor = chart[y].get(x + 1);

    if let Some(&d) = right_neighbor {
        if marked_nines.get(&(y, x + 1)).is_none() && d > curr_digit && (d - curr_digit) == 1 {
            count += foo(chart, marked_nines, (y, x + 1), d);
        }
    }

    let bottom_neighbor = chart.get(y + 1).map(|a| &a[x]);

    if let Some(&d) = bottom_neighbor {
        if marked_nines.get(&(y + 1, x)).is_none() && d > curr_digit && (d - curr_digit) == 1 {
            count += foo(chart, marked_nines, (y + 1, x), d);
        }
    }

    let left_neighbor = if x > 0 { chart[y].get(x - 1) } else { None };

    if let Some(&d) = left_neighbor {
        if marked_nines.get(&(y, x - 1)).is_none() && d > curr_digit && (d - curr_digit) == 1 {
            count += foo(chart, marked_nines, (y, x - 1), d);
        }
    }

    count
}
