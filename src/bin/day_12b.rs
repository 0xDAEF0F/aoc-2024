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
            let sides = calculate_sides(&region, &grid);
            total_score += region.len() * sides;
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

fn calculate_sides(region: &Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> usize {
    let mut perimeter = region.len() * 4;
    let mut marked_pairs = HashSet::<((usize, usize), (usize, usize))>::new();

    for &current_node @ (y, x) in region.iter() {
        let possible_directions = [
            (y.wrapping_sub(1), x), // top
            (y, x.wrapping_sub(1)), // left
            (y + 1, x),             // bottom
            (y, x + 1),             // right
        ];

        for &possible_neighbor in &possible_directions {
            if let Some(&neighbor) = region.iter().find(|&&spots| spots == possible_neighbor) {
                if !marked_pairs.contains(&(neighbor, current_node))
                    && !marked_pairs.contains(&(current_node, neighbor))
                {
                    let extra_sides_to_cut = foo(neighbor, current_node, grid);
                    perimeter -= 2 + extra_sides_to_cut;
                    marked_pairs.insert((current_node, neighbor));
                    marked_pairs.insert((neighbor, current_node));
                }
            }
        }
    }

    perimeter
}

fn foo(pair_a: (usize, usize), pair_b: (usize, usize), grid: &Vec<Vec<char>>) -> usize {
    let is_north_south = pair_a.0.abs_diff(pair_b.0) == 1;

    let mut sides_to_cut = 0;

    let (a, b, c, d) = if is_north_south {
        let neighbor_a = grid[pair_a.0].get(pair_a.1.wrapping_sub(1));
        let neighbor_b = grid[pair_a.0].get(pair_a.1 + 1);

        let neighbor_c = grid[pair_b.0].get(pair_b.1.wrapping_sub(1));
        let neighbor_d = grid[pair_b.0].get(pair_b.1 + 1);

        (neighbor_a, neighbor_b, neighbor_c, neighbor_d)
    } else {
        let neighbor_a = grid.get(pair_a.0 + 1).and_then(|row| row.get(pair_a.1));
        let neighbor_b = grid
            .get(pair_a.0.wrapping_sub(1))
            .and_then(|row| row.get(pair_a.1));

        let neighbor_c = grid.get(pair_b.0 + 1).and_then(|row| row.get(pair_b.1));
        let neighbor_d = grid
            .get(pair_b.0.wrapping_sub(1))
            .and_then(|row| row.get(pair_b.1));

        (neighbor_a, neighbor_b, neighbor_c, neighbor_d)
    };

    let char = grid[pair_a.0][pair_a.1];

    if (a, c) == (None, None) || vec![a, c].iter().all(|x| x.is_some_and(|xx| *xx != char)) {
        sides_to_cut += 1;
    }

    if (b, d) == (None, None) || vec![b, d].iter().all(|x| x.is_some_and(|xx| *xx != char)) {
        sides_to_cut += 1;
    }

    sides_to_cut
}

fn build_grid() -> Vec<Vec<char>> {
    let file = File::open("data/day_12.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec()
}
