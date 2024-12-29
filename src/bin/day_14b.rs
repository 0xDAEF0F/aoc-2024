use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct State {
    pub rows: i32,
    pub cols: i32,
    pub robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

impl State {
    pub fn advance_robots(&mut self) {
        for robot in self.robots.iter_mut() {
            let (x, y) = robot.position;
            let (vx, vy) = robot.velocity;

            let new_x = (x + vx).rem_euclid(self.cols);
            let new_y = (y + vy).rem_euclid(self.rows);

            robot.position = (new_x, new_y);
        }
    }

    pub fn print_robots(&self) {
        let mut matrix = vec![vec!['.'; self.cols as usize]; self.rows as usize];

        for robot in self.robots.iter() {
            let (x, y) = robot.position;
            matrix[y as usize][x as usize] = '*';
        }

        for (i, row) in matrix.iter().enumerate() {
            println!("{} - {i}", row.iter().collect::<String>());
        }
    }

    pub fn has_n_horizontal_neighbor_robots(&self, n: usize) -> bool {
        for row in 0..self.rows {
            let count = self
                .robots
                .iter()
                .filter(|r| r.position.1 == row)
                .map(|r| r.position.0)
                .sorted()
                .tuple_windows()
                .filter(|(a, b)| b - a == 1)
                .count();

            if count >= n {
                return true;
            }
        }

        false
    }
}

fn main() {
    let file = File::open("data/day_14.txt").unwrap();
    let reader = BufReader::new(file);

    let regex = Regex::new(r"[-]?\d+").unwrap();

    let mut state = State {
        rows: 0,
        cols: 0,
        robots: vec![],
    };

    for line in reader.lines() {
        let line = line.unwrap();

        let (x, y, vx, vy) = regex
            .find_iter(&line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        let robot = Robot {
            position: (x, y),
            velocity: (vx, vy),
        };

        state.robots.push(robot);

        if x > state.cols {
            state.cols = x + 1;
        }

        if y > state.rows {
            state.rows = y + 1;
        }
    }

    // grab stdin to advance robots and wait for 'Enter' to be pressed
    let stdin = std::io::stdin();
    let mut line = String::new();

    let mut counter = 0;

    loop {
        state.advance_robots();
        counter += 1;
        if state.has_n_horizontal_neighbor_robots(13) {
            state.print_robots();
            println!("counter: {}", counter);
            line.clear();
            stdin.read_line(&mut line).unwrap();
        }
    }
}
