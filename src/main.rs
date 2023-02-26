use std::include_str;

pub fn main() {
    println!(
        "{:?}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|elf| { elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>() })
            .max()
    );
}
