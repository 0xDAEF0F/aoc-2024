use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
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

    pub fn get_safety_factor(&self) -> usize {
        let quadrant_robots = (1..=4).map(|quadrant| match quadrant {
            1 => self.get_robots_within(0..self.cols / 2, 0..self.rows / 2),
            2 => self.get_robots_within((self.cols + 1) / 2..self.cols, 0..self.rows / 2),
            3 => self.get_robots_within(0..self.cols / 2, (self.rows + 1) / 2..self.rows),
            4 => self.get_robots_within(
                (self.cols + 1) / 2..self.cols,
                (self.rows + 1) / 2..self.rows,
            ),
            _ => unreachable!(),
        });

        quadrant_robots.product()
    }

    fn get_robots_within(&self, xr: Range<i32>, yr: Range<i32>) -> usize {
        self.robots
            .iter()
            .filter(|r| xr.contains(&r.position.0) && yr.contains(&r.position.1))
            .count()
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

    for _ in 0..100 {
        state.advance_robots();
    }

    let safety_factor = state.get_safety_factor();

    println!("safety factor: {safety_factor}");
}
