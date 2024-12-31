use anyhow::{Context, Ok, Result};

struct State {
    pub grid: Vec<Vec<char>>,
    pub directions: Vec<char>,
    pub robot_location: (usize, usize), // (y, x) || (row, col)
}

impl State {
    fn advance_robot(&mut self) {
        if let Some(direction) = self.directions.pop() {
            let delta = match direction {
                '<' => (0, -1),
                'v' => (1, 0),
                '^' => (-1, 0),
                '>' => (0, 1),
                _ => unreachable!(""),
            };
            self.move_things_recursively(self.robot_location, delta);
        }
    }

    fn move_things_recursively(
        &mut self,
        _coordinates @ (y, x): (usize, usize),
        delta @ (dy, dx): (i32, i32),
    ) {
        let neighbor @ (ny, nx) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);
        let char = self.grid[ny][nx];

        if char == '#' {
            return;
        }

        if char == 'O' {
            self.move_things_recursively(neighbor, delta);
        }

        if self.grid[ny][nx] == '.' {
            let thing_to_move = self.grid[y][x];
            self.grid[ny][nx] = thing_to_move;
            self.grid[y][x] = '.';
            if thing_to_move == '@' {
                self.robot_location = (ny, nx);
            }
        }
    }

    fn count_boxes_score(&self) -> usize {
        let boxes = self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, &ch)| {
                    if ch == 'O' {
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

    println!("{:?}", state.count_boxes_score());

    Ok(())
}

fn build_state() -> Result<State> {
    let contents = std::fs::read_to_string("data/day_15.txt")?;

    let (grid, directions) = contents.split_once("\n\n").context("split_once - 20")?;

    let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let directions: Vec<char> = directions
        .lines()
        .flat_map(|line| line.chars())
        .rev()
        .collect();

    let robot_location = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&ch| ch == '@').map(|x| (y, x)))
        .context("robot_location - 27")?;

    Ok(State {
        grid,
        directions,
        robot_location,
    })
}
