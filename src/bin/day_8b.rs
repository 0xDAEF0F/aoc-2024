use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let map = build_map();
    let mut hm = HashMap::<char, Vec<(i128, i128)>>::new();

    let num_rows = (map.len() - 1) as i128;
    let num_cols = (map[0].len() - 1) as i128;

    let mut antinodes = HashSet::new();

    for (row_idx, row) in map.into_iter().enumerate() {
        let row_idx = row_idx as i128;
        for (col_idx, char) in row.into_iter().enumerate() {
            let col_idx = col_idx as i128;
            if char == '.' {
                continue;
            }
            if let Some(vec) = hm.get_mut(&char) {
                vec.push((col_idx, row_idx));
            } else {
                hm.insert(char, vec![(col_idx, row_idx)]);
            };
        }
    }

    for (_, vec) in hm.iter() {
        let pairs = get_all_pairs(vec);
        for (antenna, antenna_2) in pairs {
            let an = get_antinode_coordinates(antenna, antenna_2, (num_rows, num_cols));

            for a in an {
                antinodes.insert(a);
            }
        }
    }

    println!("antinodes: {}", antinodes.len());

    Ok(())
}

fn can_antinode_fit(total_cols: i128, total_rows: i128, antinode: (i128, i128)) -> bool {
    let (x, y) = antinode;

    if x < 0 || y < 0 {
        return false;
    }

    if x > total_cols || y > total_rows {
        return false;
    }

    true
}

fn get_all_pairs(input: &Vec<(i128, i128)>) -> Vec<((i128, i128), (i128, i128))> {
    let mut pairs = Vec::new();

    for (i, &item1) in input.iter().enumerate() {
        for &item2 in input.iter().skip(i + 1) {
            pairs.push((item1, item2));
        }
    }

    pairs
}

fn build_map() -> Vec<Vec<char>> {
    let file = File::open("data/day_8.txt").expect("file does not exist.");
    let reader = BufReader::new(file);

    let mut vec = vec![];

    for line in reader.lines() {
        let line = line.expect("line does not exist.");

        let mut row = vec![];

        for char in line.chars() {
            row.push(char);
        }

        vec.push(row);
    }

    vec
}

fn get_antinode_coordinates(
    antenna: (i128, i128),
    antenna_2: (i128, i128),
    (cols, rows): (i128, i128),
) -> Vec<(i128, i128)> {
    let (x, y) = antenna;
    let (x2, y2) = antenna_2;

    let (x_, y_) = (x - x2, y - y2);
    let (x__, y__) = (x2 - x, y2 - y);

    let mut antinodes = vec![];

    antinodes.push(antenna);
    antinodes.push(antenna_2);

    let mut maybe_antinode = (x + x_, y + y_);
    let mut maybe_antinode2 = (x2 + x__, y2 + y__);

    loop {
        if can_antinode_fit(cols, rows, maybe_antinode) {
            antinodes.push(maybe_antinode);
            let (x, y) = maybe_antinode;
            maybe_antinode = (x + x_, y + y_);
        } else {
            break;
        }
    }

    loop {
        if can_antinode_fit(cols, rows, maybe_antinode2) {
            antinodes.push(maybe_antinode2);
            let (x, y) = maybe_antinode2;
            maybe_antinode2 = (x + x__, y + y__);
        } else {
            break;
        }
    }

    antinodes
}
