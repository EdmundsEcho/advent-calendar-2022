// use chomp::ascii::decimal;
// use chomp::prelude::*;

use std::str;
use std::str::Utf8Error;

use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

// #[macro_use]
extern crate chomp;

// move 1 from 7 to 6
#[derive(Debug)]
struct Move {
    number_crates: usize,
    from_stack: usize,
    to_stack: usize,
}
fn parse_number(input: &[u8]) -> Result<usize, std::num::ParseIntError> {
    let input = str::from_utf8(input).unwrap();
    input.parse()
}
fn parse_move(input: &[u8]) -> IResult<&[u8], Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number_crates) = map_res(digit1, parse_number)(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_stack) = map_res(digit1, parse_number)(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_stack) = map_res(digit1, parse_number)(input)?;

    Ok((
        input,
        Move {
            number_crates,
            from_stack: from_stack - 1,
            to_stack: to_stack - 1,
        },
    ))
}
/*
fn parse_move<I: U8Input>(i: I) -> SimpleResult<I, Move> {
    parse! {i;
            string(b"move ");
        let crates = decimal() <* token(b' ');
            string(b"from ");
        let from = decimal() <* token(b' ');
            string(b"to ");
        let to = decimal();

        ret Move {
            number_crates: crates,
            from_stack: from - 1,
            to_stack: to - 1,
        }
    }
} */

type Stacks = Vec<Vec<u8>>;

fn apply_move(start_value: &mut Stacks, move_value: Move) -> Result<(), String> {
    // one at a time, pop from, push to
    let mut buf: u8;
    for _ in 0..move_value.number_crates {
        buf = start_value[move_value.from_stack]
            .pop()
            .ok_or("Tried to pop from an empty stack")?;
        start_value[move_value.to_stack].push(buf);
    }
    Ok(())
}

//    [P]                 [Q]     [T]
fn read_nine(data: &[u8]) -> [u8; 9] {
    let mut result = [0u8; 9];
    let positions = vec![1, 5, 9, 13, 17, 21, 25, 29, 33];
    for (i, p) in positions.iter().enumerate() {
        result[i] = data[*p];
    }
    result
}

// 9 columns x 8 rows
// first  = (9 x 8) - 8
// second = (9 x 7) - 8
// third  = (9 x 6) - 8
// or
// 0 -> 0,8
// 1 -> 1,8
// 2 -> 2,8
// 3 -> 3,8
//
fn build_stack<'a>(stack_number: usize, raw_input: &Vec<&'a [u8]>) -> Vec<u8> {
    let to_stack = stack_number - 1;
    let sequence = (0..=8).rev();
    let mut stack = Vec::new();
    for stack_item in sequence {
        // pivot of sorts happens here
        stack.push(raw_input[stack_item][to_stack]);
    }
    stack.into_iter().filter(|x| *x != b' ').collect()
}

fn get_answer<'a>(stacks: &Stacks, buffer: &'a mut [u8; 9]) -> Result<&'a str, Utf8Error> {
    for i in 0..9 {
        let last = stacks[i].len() - 1;
        buffer[i] = stacks[i][last];
    }
    std::str::from_utf8(buffer)
}

pub fn main() {
    let raw_start = include_bytes!("input_start.txt")
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| read_nine(line))
        .collect::<Vec<_>>();

    let mut stacks = Vec::with_capacity(9);
    // stack number is 1 - 9
    for stack in 1..=9 {
        stacks.push(build_stack(
            stack,
            &raw_start.iter().map(|x| x.as_slice()).collect(),
        ))
    }

    // process stack moves
    let stacks = &mut stacks;
    let _ = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, mv) = parse_move(line).unwrap();
            apply_move(stacks, mv).unwrap();
        })
        .collect::<Vec<_>>();

    // read the last crate in each stack
    let mut answer = [0_u8; 9];
    let answer = get_answer(&stacks, &mut answer);
    println!("Raw start: {:?}", raw_start);
    println!("Stacks: {:?}", &stacks);
    // println!("Moves: {:?}", &moves);
    println!("Final Answer: {:?}", &answer);
}
