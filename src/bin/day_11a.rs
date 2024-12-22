fn main() {
    let initial_stones: Vec<u128> = vec![8069, 87014, 98, 809367, 525, 0, 9494914, 5];

    let final_stones = (0..25).fold(initial_stones, |acc, _| {
        acc.iter()
            .flat_map(|&num| {
                if num == 0 {
                    vec![1].into_iter()
                } else if num.to_string().len() % 2 == 0 {
                    let num_str = num.to_string();
                    let mid = num_str.len() / 2;
                    let (fst, snd) = num_str.split_at(mid);
                    vec![fst.parse().unwrap(), snd.parse().unwrap()].into_iter()
                } else {
                    vec![num * 2024].into_iter()
                }
            })
            .collect::<Vec<u128>>()
    });

    println!("{:?}", final_stones.len());
}
