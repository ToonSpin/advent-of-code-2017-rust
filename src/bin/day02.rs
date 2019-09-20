use std::io;
use std::io::prelude::*;

use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, verify},
    multi::separated_list,
    IResult,
};

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<u32>> {
    let f = |v: &Vec<u32>| v.len() > 0;
    verify(separated_list(char('\t'), parse_u32), f)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = separated_list(char('\n'), parse_row)(input).unwrap();

    let mut checksum = 0;
    let mut result_sum = 0;
    for v in input.iter() {
        let mut min = std::u32::MAX;
        let mut max = std::u32::MIN;

        for i in v.iter() {
            if *i < min {
                min = *i;
            }
            if *i > max {
                max = *i;
            }
        }

        checksum += max - min;

        let mut sorted = v.clone();
        sorted.sort();
        'result_loop: for i in 0..sorted.len() {
            for j in i + 1..sorted.len() {
                if sorted[j] % sorted[i] == 0 {
                    result_sum += sorted[j] / sorted[i];
                    break 'result_loop;
                }
            }
        }
    }

    println!("The spreadsheet's checksum is: {}", checksum);
    println!("The sum of each row's result is: {}", result_sum);

    Ok(())
}
