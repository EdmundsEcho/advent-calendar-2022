// split the input in half
// find matching values in each side
// compute the priority of each
pub fn main() {
    let result = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|sack| sack.split_at(sack.len() / 2))
        .map(|(a, b)| b.iter().filter(|b| a.contains(b)).collect::<Vec<_>>())
        .filter(|matches| !matches.is_empty())
        .map(|matches| {
            let b = matches[0];
            if *b >= b'a' {
                (b - b'a') as u32 + 1
            } else {
                (b - b'A') as u32 + 27
            }
        })
        .sum::<u32>();

    println!("a: {}", b'a');
    println!("A: {}", b'A');
    println!("Answer: {}", result);
}
