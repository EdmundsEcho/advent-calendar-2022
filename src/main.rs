// process 3 lines in a batch
// find matching values in all 3
// compute the priority of each
use itertools::Itertools;
use std::collections::HashSet;

pub fn main() {
    let chunks = include_bytes!("input.txt").split(|b| *b == b'\n').chunks(3);
    let result = chunks
        .into_iter()
        // .inspect(|x| println!("{:?}", x.count()));
        .map(|chunk| {
            // find a way to identify the first char found in each of the
            // 3 lines in the chunk
            let hash_sets = chunk
                .map(|line| line.iter().collect::<HashSet<_>>())
                .collect::<Vec<HashSet<_>>>();

            let mut result = hash_sets[0].clone();
            for hash_set in hash_sets.iter().skip(1) {
                result = result.intersection(hash_set).copied().collect();
            }
            result
        })
        .filter(|hs| !hs.is_empty())
        /*
        .inspect(|x| {
            println!(
                "{:?}",
                String::from_utf8(x.iter().cloned().map(|&b| b).collect::<Vec<u8>>())
            )
        }) */
        .map(|set| set.iter().next().cloned()) // convert hashset -> option<value>
        .filter_map(|x| x)
        .map(|matched| {
            let b = matched;
            if b >= &b'a' {
                (b - b'a') as u32 + 1
            } else {
                (b - b'A') as u32 + 27
            }
        })
        .sum::<u32>();

    println!("Answer: {:?}", result);
}
