use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};
use Direction::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    East,
    South,
    West,
    North,
}

fn djikstras(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Option<Vec<Vec<(usize, usize)>>> {
    let mut pq: PriorityQueue<_, _> = PriorityQueue::from(vec![((start, East), Reverse(0))]);
    let mut visited_nodes = HashSet::new();
    let mut paths = HashMap::from([((start, East), vec![vec![start]])]);

    while let Some((((y, x), direction), Reverse(count))) = pq.pop() {
        if !visited_nodes.insert(((y, x), direction)) {
            continue;
        };

        if grid[y][x] == 'E' {
            return Some(paths[&((y, x), direction)].clone());
        }

        let neighbors = [
            (y - 1, x, North),
            (y, x + 1, East),
            (y + 1, x, South),
            (y, x - 1, West),
        ];

        let free_neighbors = neighbors
            .into_iter()
            .filter(|&(y, x, d)| (grid[y][x] == '.' || grid[y][x] == 'E'))
            .collect_vec();

        for (ny, nx, n_dir) in free_neighbors {
            let cost = get_relative_cost(direction, n_dir);
            let new_cost = count + cost;

            if !visited_nodes.contains(&((ny, nx), n_dir)) {
                if let Some(&Reverse(priority)) = pq.get_priority(&((ny, nx), n_dir)) {
                    if new_cost < priority {
                        pq.change_priority(&((ny, nx), n_dir), Reverse(new_cost));
                        paths.insert(
                            ((ny, nx), n_dir),
                            paths[&((y, x), direction)]
                                .clone()
                                .into_iter()
                                .map(|mut v| {
                                    v.push((ny, nx));
                                    v
                                })
                                .collect(),
                        );
                    } else if new_cost == priority {
                        let mut additional_paths = paths[&((y, x), direction)].clone();
                        for path in &mut additional_paths {
                            path.push((ny, nx));
                        }
                        if let Some(existing_paths) = paths.get_mut(&((ny, nx), n_dir)) {
                            existing_paths.extend(additional_paths);
                        } else {
                            paths.insert(((ny, nx), n_dir), additional_paths);
                        }
                    }
                } else {
                    pq.push(((ny, nx), n_dir), Reverse(new_cost));
                    let mut new_paths = paths[&((y, x), direction)].clone();
                    for path in &mut new_paths {
                        path.push((ny, nx));
                    }
                    paths.insert(((ny, nx), n_dir), new_paths);
                }
            }
        }
    }

    None
}

fn get_relative_cost(curr_direction: Direction, to: Direction) -> u32 {
    match curr_direction {
        North => match to {
            North => 1,
            East | West => 1001,
            South => 2001,
        },
        East => match to {
            East => 1,
            South | North => 1001,
            West => 2001,
        },
        South => match to {
            South => 1,
            West | East => 1001,
            North => 2001,
        },
        West => match to {
            West => 1,
            North | South => 1001,
            East => 2001,
        },
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/day_16.txt").unwrap();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_location = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&ch| ch == 'S').map(|x| (y, x)))
        .unwrap();

    if let Some(best_score) = djikstras(&grid, start_location) {
        let hs: HashSet<(usize, usize)> = best_score.into_iter().flatten().collect();
        println!("min score: {:?}", hs.len());
    } else {
        println!("no path found");
    }
}
