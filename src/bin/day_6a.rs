use anyhow::Result;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("data/day_6.txt")?;
    let reader = BufReader::new(file);

    let mut unique_positions = HashSet::<(usize, usize)>::new();
    let mut vec: Vec<Vec<char>> = vec![];
    let mut current_coordinates: (usize, usize) = (0, 0);
    let mut current_direction = "north";

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row: Vec<char> = vec![];
        for (j, ch) in line.chars().enumerate() {
            if ch == '^' {
                current_coordinates = (i, j);
                unique_positions.insert((i, j));
                row.push('.');
                continue;
            }
            row.push(ch);
        }
        vec.push(row);
    }

    loop {
        let (i, j) = current_coordinates;
        let _next_direction @ (i, j) = match current_direction {
            "north" => (i - 1, j),
            "east" => (i, j + 1),
            "south" => (i + 1, j),
            "west" => (i, j - 1),
            _ => panic!(""),
        };
        if let Some(row) = vec.get(i) {
            if let Some(&char) = row.get(j) {
                if char == '.' {
                    unique_positions.insert((i, j));
                    current_coordinates = (i, j);
                } else if char == '#' {
                    let next_direction = match current_direction {
                        "north" => "east",
                        "east" => "south",
                        "south" => "west",
                        "west" => "north",
                        _ => panic!(""),
                    };
                    current_direction = next_direction;
                } else {
                    panic!("never")
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    println!("Unique movements: {}", unique_positions.len());

    Ok(())
}
