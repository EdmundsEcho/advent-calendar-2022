use std::include_str;

// top 3
pub fn main() {
    let mut vec = include_str!("input.txt")
        .split("\n\n")
        .map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    vec.sort_by(|a, b| b.cmp(a));

    let temp: u32 = vec.iter().take(3).sum();

    println!("{:?}", temp);
}
