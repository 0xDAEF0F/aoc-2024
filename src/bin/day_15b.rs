use anyhow::{Context, Ok, Result};

struct State {
    pub grid: Vec<Vec<char>>,
    pub directions: Vec<Direction>,
    pub robot_location: (usize, usize), // (y, x) || (row, col)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Bottom,
}

impl State {
    fn advance_robot(&mut self) {
        if let Some(direction) = self.directions.pop() {
            let (y, x) = self.robot_location;

            // check the space where the robot is supposed to be pushed:
            let (ny, nx) = match direction {
                Direction::Left => (y, x - 1),
                Direction::Up => (y - 1, x),
                Direction::Right => (y, x + 1),
                Direction::Bottom => (y + 1, x),
            };

            // - if '#' return
            if self.grid[ny][nx] == '#' {
                return;
            }

            // - if '.' then just move it and return
            if self.grid[ny][nx] == '.' {
                self.grid[ny][nx] = '@';
                self.grid[y][x] = '.';
                self.robot_location = (ny, nx);
                return;
            }

            // - if '[' or ']'
            if self.grid[ny][nx] == '[' || self.grid[ny][nx] == ']' {
                let box_to_move = if self.grid[ny][nx] == '[' {
                    (ny, nx)
                } else {
                    (ny, nx - 1)
                };
                //  - can the box(es) be moved?
                let mut grid_clone = self.grid.clone();
                if let false = State::foo(box_to_move, direction, &mut grid_clone) {
                    return;
                }
                self.grid = grid_clone;
                self.grid[ny][nx] = '@';
                self.grid[y][x] = '.';
                self.robot_location = (ny, nx);
            }
        }
    }

    fn foo((y, x): (usize, usize), direction: Direction, grid: &mut Vec<Vec<char>>) -> bool {
        let where_to_check = match direction {
            Direction::Up => vec![(y - 1, x), (y - 1, x + 1)],
            Direction::Bottom => vec![(y + 1, x), (y + 1, x + 1)],
            Direction::Left => vec![(y, x - 1)],
            Direction::Right => vec![(y, x + 2)],
        };

        // # wall path
        if where_to_check.iter().any(|&(y, x)| grid[y][x] == '#') {
            return false;
        }

        // [], ][, .[, ]. -- Up, Down path
        if let [(ny, nx), (ny2, nx2)] = where_to_check.clone()[..] {
            let concatenated = format!("{}{}", grid[ny][nx], grid[ny2][nx2]);
            let res = match concatenated.as_str() {
                "[]" => State::foo((ny, nx), direction, grid),
                "][" => {
                    State::foo((ny, nx - 1), direction, grid)
                        && State::foo((ny, nx + 1), direction, grid)
                }
                ".[" => State::foo((ny, nx + 1), direction, grid),
                "]." => State::foo((ny, nx - 1), direction, grid),
                _ => true,
            };

            if !res {
                return false;
            }
        }

        // right, left path
        if let [(ny, nx)] = where_to_check.clone()[..] {
            let res = match grid[ny][nx] {
                '[' => State::foo((ny, nx), direction, grid),
                ']' => State::foo((ny, nx - 1), direction, grid),
                _ => true,
            };

            if !res {
                return false;
            }
        }

        // . all clear path
        if where_to_check.iter().all(|&(y, x)| grid[y][x] == '.') {
            match direction {
                Direction::Left => {
                    grid[y][x - 1] = '[';
                    grid[y][x] = ']';
                    grid[y][x + 1] = '.';
                }
                Direction::Right => {
                    grid[y][x] = '.';
                    grid[y][x + 1] = '[';
                    grid[y][x + 2] = ']';
                }
                Direction::Bottom => {
                    grid[y + 1][x] = '[';
                    grid[y + 1][x + 1] = ']';
                    grid[y][x] = '.';
                    grid[y][x + 1] = '.';
                }
                Direction::Up => {
                    grid[y - 1][x] = '[';
                    grid[y - 1][x + 1] = ']';
                    grid[y][x] = '.';
                    grid[y][x + 1] = '.';
                }
            };
        }

        return true;
    }

    fn count_boxes_score(&self) -> usize {
        let boxes = self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, &ch)| {
                    if ch == '[' {
                        Some((y, x))
                    } else {
                        None
                    }
                },
            )
        });
        let scores = boxes.map(|(y, x)| 100 * y + x);

        scores.sum()
    }
}

fn main() -> Result<()> {
    let mut state = build_state()?;

    while !state.directions.is_empty() {
        state.advance_robot();
    }

    println!("score: {}", state.count_boxes_score());

    Ok(())
}

fn build_state() -> Result<State> {
    let contents = std::fs::read_to_string("data/day_15.txt")?;

    let (grid, directions) = contents.split_once("\n\n").context("split_once")?;

    let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let grid = transform_grid(grid);

    let directions: Vec<Direction> = directions
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '<' => Direction::Left,
                'v' => Direction::Bottom,
                '^' => Direction::Up,
                '>' => Direction::Right,
                _ => panic!("can not parse direction"),
            })
        })
        .rev()
        .collect();

    let robot_location = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&ch| ch == '@').map(|x| (y, x)))
        .context("robot_location")?;

    Ok(State {
        grid,
        directions,
        robot_location,
    })
}

fn transform_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|ch| match ch {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![],
                })
                .collect()
        })
        .collect()
}
