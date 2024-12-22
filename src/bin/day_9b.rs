#[derive(Debug, Clone, Copy)]
struct Block {
    space: usize,
    digit: Option<usize>,
}

fn main() {
    let input = include_str!("../../data/day_9.txt");

    let mut block: Vec<Block> = input
        .chars()
        .filter(|&c| c.is_digit(10))
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 0 {
                Block {
                    space: d,
                    digit: Some(i / 2),
                }
            } else {
                Block {
                    space: d,
                    digit: None,
                }
            }
        })
        .collect();

    let mut i = block.len() - 1;

    while i > 0 {
        if block[i].digit.is_none() {
            i -= 1;
            continue;
        }

        let fit_idx = block[..i]
            .iter()
            .enumerate()
            .filter(|b| b.1.digit.is_none())
            .find(|b| b.1.space >= block[i].space);

        if let Some((idx, b)) = fit_idx {
            if b.space > block[i].space {
                let extra_block = Block {
                    space: b.space - block[i].space,
                    digit: None,
                };
                block[idx].space = block[i].space;
                block.swap(idx, i);
                block.insert(idx + 1, extra_block);
            } else if b.space == block[i].space {
                block.swap(idx, i);
            }
        } else {
            i -= 1;
        }
    }

    let count: usize = block
        .iter()
        .flat_map(|b| {
            if let Some(d) = b.digit {
                vec![Some(d); b.space]
            } else {
                vec![None; b.space]
            }
        })
        .enumerate()
        .filter_map(|(idx, d)| d.map(|d| d * idx))
        .sum();

    println!("{}", count);
}
