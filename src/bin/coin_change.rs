use std::{collections::HashMap, time::Instant};

fn main() {
    let coins = [1, 5, 10, 25, 50];
    let target = 1000;

    let start = Instant::now();
    println!("ways: {}", foo(&coins, target));
    let duration = start.elapsed();

    println!("Time elapsed in a is: {:?}", duration);

    // ----------

    let mut hm = HashMap::new();

    let start = Instant::now();
    println!("ways_b: {}", foo_dynamic(coins.to_vec(), target, &mut hm));
    let duration = start.elapsed();

    println!("Time elapsed in b is: {:?}", duration);
}

fn foo(coins: &[i32], target: i32) -> i32 {
    if target == 0 {
        return 1;
    }

    if let Some((head, rest)) = coins.split_first() {
        if target >= *head {
            return foo(coins, target - head) + foo(rest, target);
        } else {
            return 0;
        }
    }

    0
}

fn foo_dynamic(coins: Vec<i32>, target: i32, hm: &mut HashMap<(Vec<i32>, i32), i32>) -> i32 {
    if let Some(&ways) = hm.get(&(coins.clone(), target)) {
        return ways;
    }

    if target == 0 {
        return 1;
    }

    if let Some((head, rest)) = coins.split_first() {
        if target >= *head {
            let fst = foo_dynamic(coins.clone(), target - head, hm);
            hm.insert((coins.clone(), target - head), fst);

            let snd = foo_dynamic(rest.to_vec(), target, hm);
            hm.insert((rest.to_vec(), target), snd);

            return fst + snd;
        } else {
            return 0;
        }
    }

    0
}
