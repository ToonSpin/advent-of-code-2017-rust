use std::io;
use std::io::prelude::*;

fn reverse_section(mut v: Vec<u8>, section_start: usize, section_length: usize) -> Vec<u8> {
    let max: usize = section_length / 2;
    let len = v.len();

    for i in 0..max {
        let p = section_start + i;
        let q = section_start + section_length - 1 - i;

        let temp = v[p % len];
        v[p % len] = v[q % len];
        v[q % len] = temp;
    }

    v
}

fn hash_rounds(input: &Vec<u8>, rounds: u8) -> Vec<u8> {
    let mut skip_size = 0;
    let mut start = 0;
    let mut numbers = vec![];

    for i in 0..=255 {
        numbers.push(i);
    }

    for _round in 0..rounds {
        for i in input.iter() {
            numbers = reverse_section(numbers, start, *i as usize);
            start += *i as usize + skip_size;
            start %= 256;
            skip_size += 1;
        }
    }

    numbers
}

fn get_dense_hash(input: &str) -> Vec<u8> {
    let mut input: Vec<u8> = input.as_bytes().iter().map(|b| *b as u8).collect();
    input.append(&mut vec![17, 31, 73, 47, 23]);

    let mut result = Vec::new();
    let sparse_hash = hash_rounds(&input, 64);
    for c in sparse_hash.chunks(16) {
        result.push(c.iter().fold(0, |a, e| a ^ e));
    }
    result
}

fn knot_hash(input: &str) -> String {
    let mut s = String::new();
    let chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let v = get_dense_hash(input);

    for i in v.iter() {
        let i = *i as usize;

        let second_char = i % 16;
        let first_char = (i / 16) % 16;

        s.push(chars[first_char]);
        s.push(chars[second_char]);
    }

    s
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let input_part1: Vec<u8> = input
        .split(',')
        .map(|s| str::parse::<u8>(s).unwrap())
        .collect();
    let numbers = hash_rounds(&input_part1, 1);

    println!(
        "The product of the first two numbers after the first round: {}",
        numbers[0] as u16 * numbers[1] as u16
    );
    println!("The knot hash of the input is: {}", knot_hash(input));

    Ok(())
}
