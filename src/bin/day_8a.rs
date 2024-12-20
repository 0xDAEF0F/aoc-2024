use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let map = build_map();
    let mut hm = HashMap::<char, Vec<(usize, usize)>>::new();

    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut antinodes = HashSet::new();

    for (row_idx, row) in map.into_iter().enumerate() {
        for (col_idx, char) in row.into_iter().enumerate() {
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
            let (a1, a2) = get_antinode_coordinates(antenna, antenna_2);

            if can_antinode_fit(num_cols, num_rows, a1) {
                antinodes.insert(a1);
            }

            if can_antinode_fit(num_cols, num_rows, a2) {
                antinodes.insert(a2);
            }
        }
    }

    println!("antinodes: {}", antinodes.len());

    Ok(())
}

fn can_antinode_fit(total_cols: usize, total_rows: usize, antinode: (i128, i128)) -> bool {
    let (x, y) = antinode;

    if x < 0 || y < 0 {
        return false;
    }

    if x > total_cols as i128 - 1 || y > total_rows as i128 - 1 {
        return false;
    }

    true
}

fn get_all_pairs(input: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
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
    antenna: (usize, usize),
    antenna_2: (usize, usize),
) -> ((i128, i128), (i128, i128)) {
    let (x, y) = antenna;
    let (x2, y2) = antenna_2;

    let (x_, y_) = (x as i128 - x2 as i128, y as i128 - y2 as i128);
    let (x__, y__) = (x2 as i128 - x as i128, y2 as i128 - y as i128);

    (
        (x as i128 + x_, y as i128 + y_),
        (x2 as i128 + x__, y2 as i128 + y__),
    )
}

#[cfg(test)]
mod test {
    use crate::get_antinode_coordinates;

    #[test]
    fn a() {
        let coord_a: (usize, usize) = (5, 2);
        let coord_b: (usize, usize) = (7, 3);

        let (antinode, antinode_b) = get_antinode_coordinates(coord_a, coord_b);

        assert_eq!(antinode, (3, 1));
        assert_eq!(antinode_b, (9, 4));
    }
}
