use std::collections::VecDeque;

fn main() {
    let now = std::time::Instant::now();

    let input: Vec<(usize, usize)> = include_str!("../../data/day_18.txt")
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut last = 1025;
    loop {
        if last > input.len() {
            break;
        }

        let mut grid = [['.'; 71]; 71];

        for &(x, y) in input.iter().take(last) {
            grid[y][x] = '#';
        }

        if bfs(&grid, (0, 0)) == None {
            break;
        }

        last += 1;
    }

    println!("last obstacle to impede passing: {:?}", input[last - 1]);
    println!("time elapsed: {}ms", now.elapsed().as_millis());
}

fn bfs(grid: &[[char; 71]; 71], start @ (y, x): (usize, usize)) -> Option<usize> {
    let mut visited = [[false; 71]; 71];
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited[y][x] = true;

    while let Some(((y, x), count)) = queue.pop_front() {
        if (y, x) == (70, 70) {
            return Some(count);
        }
        for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if ny < 0 || nx < 0 || ny > 70 || nx > 70 {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if visited[ny][nx] || grid[ny][nx] == '#' {
                continue;
            }
            queue.push_back(((ny, nx), count + 1));
            visited[ny][nx] = true;
        }
    }

    None
}
