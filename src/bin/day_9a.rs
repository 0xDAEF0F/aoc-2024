fn main() {
    let input = include_str!("../../data/day_9.txt");

    let mut block: Vec<Option<usize>> = input
        .chars()
        .filter(|&c| c.is_digit(10))
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .enumerate()
        .flat_map(|(i, d)| {
            if i % 2 == 0 {
                vec![Some(i / 2); d]
            } else {
                vec![None; d]
            }
        })
        .collect();

    let mut i = 0;
    let mut j = block.len() - 1;

    while i < j {
        if block[i].is_some() {
            i += 1;
            continue;
        }

        if block[j].is_none() {
            j -= 1;
            continue;
        }

        block.swap(i, j);
    }

    let checksum: usize = block
        .iter()
        .enumerate()
        .filter_map(|(i, &digit)| digit.map(|d| d * i))
        .sum();

    println!("{}", checksum);
}
