use std::io;
use std::io::prelude::*;

use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res},
    multi::separated_list,
    sequence::{preceded, separated_pair},
    IResult,
};

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}
use Instruction::*;

impl Instruction {
    fn process(&self, mut programs: Vec<char>) -> Vec<char> {
        match *self {
            Spin(s) => {
                let pos = programs.len() - s;
                let mut temp = programs.drain(0..pos).collect();
                programs.append(&mut temp);
            }
            Exchange(p, q) => {
                let temp = programs[p];
                programs[p] = programs[q];
                programs[q] = temp;
            }
            Partner(p, q) => {
                let mut i = 0;
                let mut j = 0;
                while programs[i] != p {
                    i += 1;
                }
                while programs[j] != q {
                    j += 1;
                }
                let temp = programs[i];
                programs[i] = programs[j];
                programs[j] = temp;
            }
        }
        programs
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}

fn parse_spin(input: &str) -> IResult<&str, Instruction> {
    let f = |s: usize| Spin(s);
    map(preceded(char('s'), parse_usize), f)(input)
}

fn parse_exchange(input: &str) -> IResult<&str, Instruction> {
    let f = |(p, q): (usize, usize)| Exchange(p, q);
    let parser = separated_pair(parse_usize, char('/'), parse_usize);
    map(preceded(char('x'), parser), f)(input)
}

fn parse_program(input: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnop")(input)
}

fn parse_partner(input: &str) -> IResult<&str, Instruction> {
    let f = |(p, q): (char, char)| Partner(p, q);
    let parser = separated_pair(parse_program, char('/'), parse_program);
    map(preceded(char('p'), parser), f)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_spin, parse_exchange, parse_partner))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list(char(','), parse_instruction)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let input = parse_instructions(input).unwrap().1;

    let mut programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let mut solutions = Vec::new();
    let mut iteration = 0;
    let mut done = false;

    while !done {
        for i in input.iter() {
            programs = i.process(programs);
        }

        let output: String = programs.iter().collect();
        if iteration == 0 {
            println!("Order after first dance: {}", output);
        }

        if output == "abcdefghijklmnop" {
            done = true;
        }

        iteration += 1;
        solutions.push(output);
    }

    println!(
        "Order after one billion dances: {}",
        solutions[999_999_999 % solutions.len()]
    );

    Ok(())
}
