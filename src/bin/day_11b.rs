use std::collections::HashMap;

fn main() {
    let list: Vec<u128> = vec![8069, 87014, 98, 809367, 525, 0, 9494914, 5];

    let mut cache = HashMap::<(u128, u128), u128>::new();
    let mut count = 0;

    for (_, num) in list.into_iter().enumerate() {
        count += foo(num, 75, &mut cache);
    }

    println!("{:?}", count);
}

fn foo(num: u128, count: u128, cache: &mut HashMap<(u128, u128), u128>) -> u128 {
    if count == 0 {
        return 1;
    }

    if let Some(&result_cache) = cache.get(&(num, count)) {
        return result_cache;
    } else {
        let result = match num {
            0 => foo(1, count - 1, cache),
            n => {
                let to_str = n.to_string();
                let len = to_str.len();
                if len % 2 == 0 {
                    let mid = len / 2;
                    let (fst, snd) = to_str.split_at(mid);
                    let (fst, snd) = (fst.parse().unwrap(), snd.parse().unwrap());

                    let fst_result = foo(fst, count - 1, cache);
                    let snd_result = foo(snd, count - 1, cache);

                    fst_result + snd_result
                } else {
                    foo(num * 2024, count - 1, cache)
                }
            }
        };

        cache.insert((num, count), result);
        return result;
    }
}
