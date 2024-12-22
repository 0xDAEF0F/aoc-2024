use anyhow::Result;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() -> Result<()> {
    let file = File::open("data/day_6.txt")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];

    // fill the map
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row: Vec<char> = vec![];
        for (j, ch) in line.chars().enumerate() {
            row.push(ch);
        }
        map.push(row);
    }

    // find initial coordinates
    let initial_coordinates = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &ch)| ((i, j), ch)))
        .find(|(coord, ch)| *ch == '^')
        .expect("Did not found '^'")
        .0;

    // where put initial obstacle
    let initial_obstacle = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &ch)| ((i, j), ch)))
        .find(|(coord, ch)| *ch == '.')
        .expect("Did not found '.'")
        .0;

    let mut map = map;

    let mut current_coordinates = initial_coordinates;
    let mut current_direction = Direction::North;

    let mut initial_obstacle = initial_obstacle;

    let mut unique_positions = HashSet::<(usize, usize, Direction)>::new();

    // loop {
    //     let (i, j) = current_coordinates;
    //     let (i, j) = match current_direction {
    //         "north" => (i - 1, j),
    //         "east" => (i, j + 1),
    //         "south" => (i + 1, j),
    //         "west" => (i, j - 1),
    //         _ => panic!(""),
    //     };
    //     if let Some(row) = map.get(i) {
    //         if let Some(&char) = row.get(j) {
    //             if char == '.' {
    //                 unique_positions.insert((i, j));
    //                 current_coordinates = (i, j);
    //             } else if char == '#' {
    //                 let next_direction = match current_direction {
    //                     "north" => "east",
    //                     "east" => "south",
    //                     "south" => "west",
    //                     "west" => "north",
    //                     _ => panic!(""),
    //                 };
    //                 current_direction = next_direction;
    //             } else {
    //                 panic!("never")
    //             }
    //         } else {
    //             break;
    //         }
    //     } else {
    //         break;
    //     }
    // }

    println!("Unique movements: {}", unique_positions.len());

    Ok(())
}
