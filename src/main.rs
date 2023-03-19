// process 3 lines in a batch
// find matching values in all 3
// compute the priority of each
use chomp::ascii::decimal;
use chomp::prelude::*;

#[macro_use]
extern crate chomp;

fn parse_range<I: U8Input>(i: I) -> SimpleResult<I, (u32, u32)> {
    // a simulated do notation
    parse! {i;
    let low = decimal() <* token(b'-');
    let high = decimal();
        ret (low, high)
    }
}

fn is_subset(r1: (u32, u32), r2: (u32, u32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1
}
fn is_overlap(r1: (u32, u32), r2: (u32, u32)) -> bool {
    !(r1.1 < r2.0 || r2.1 < r1.0)
}

pub fn main() {
    let binding = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|line| line.split(|b| *b == b','))
        .flatten()
        .collect::<Vec<_>>()
        .chunks(2)
        // .inspect(|x| println!("chunk: {:?}", x))
        .map(|ranges| {
            ranges
                .iter()
                .map(|range| parse_only(parse_range, range))
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
        })
        .filter(|ranges| !ranges.is_empty())
        .filter(|ranges| is_overlap(ranges[0], ranges[1]))
        .count();
    // .collect::<Vec<_>>();

    println!("Answer: {:?}", binding);
}
