use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut visited_spots = HashSet::new();
    let grid = build_grid();

    let mut total_score = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited_spots.contains(&(i, j)) {
                continue;
            }
            let region = dfs((i, j), &grid, &mut visited_spots);
            let perimeter = calculate_perimeter(&region);
            total_score += region.len() * perimeter;
        }
    }

    println!("total score: {total_score}");
}

fn dfs(
    current_position: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited_spots: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut part_of_path = if !visited_spots.contains(&current_position) {
        visited_spots.insert(current_position);
        vec![current_position]
    } else {
        vec![]
    };

    let (y, x) = current_position;
    let current_char = grid[y][x];

    let possible_neighbors = [
        (y.wrapping_sub(1), x), // top
        (y, x.wrapping_sub(1)), // left
        (y + 1, x),             // bottom
        (y, x + 1),             // right
    ];

    let neighbors = possible_neighbors
        .into_iter()
        .filter(|&(ny, nx)| {
            grid.get(ny).and_then(|row| row.get(nx)) == Some(&current_char)
                && !visited_spots.contains(&(ny, nx))
        })
        .collect_vec();

    if neighbors.is_empty() {
        return part_of_path;
    }

    let visited_nodes = neighbors
        .into_iter()
        .flat_map(|neighbor| dfs(neighbor, grid, visited_spots));

    part_of_path.extend(visited_nodes);

    part_of_path
}

fn calculate_perimeter(region: &Vec<(usize, usize)>) -> usize {
    let mut perimeter = region.len() * 4;
    let mut marked_pairs = HashSet::<((usize, usize), (usize, usize))>::new();

    for &current_node @ (y, x) in region.iter() {
        let directions = [
            (y.wrapping_sub(1), x), // top
            (y, x.wrapping_sub(1)), // left
            (y + 1, x),             // bottom
            (y, x + 1),             // right
        ];

        for &possible_neighbor in &directions {
            if let Some(&neighbor) = region.iter().find(|&&spots| spots == possible_neighbor) {
                if !marked_pairs.contains(&(neighbor, current_node))
                    && !marked_pairs.contains(&(current_node, neighbor))
                {
                    perimeter -= 2;
                    marked_pairs.insert((current_node, neighbor));
                    marked_pairs.insert((neighbor, current_node));
                }
            }
        }
    }

    perimeter
}

fn build_grid() -> Vec<Vec<char>> {
    let file = File::open("data/day_12.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec()
}
