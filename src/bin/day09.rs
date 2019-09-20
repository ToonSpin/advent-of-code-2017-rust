use std::io;
use std::io::prelude::*;

use nom::{
    branch::alt,
    character::complete::{anychar, char as parse_char, none_of},
    combinator::{map, value},
    multi::{many0, separated_list},
    sequence::{delimited, preceded},
    IResult,
};

enum GroupElement {
    Group(Vec<GroupElement>),
    Garbage(u32),
}

impl GroupElement {
    fn score(&self, base_score: u32) -> u32 {
        match self {
            GroupElement::Garbage(_i) => 0,
            GroupElement::Group(v) => {
                let mut result = base_score;
                for e in v {
                    result += e.score(base_score + 1);
                }
                result
            }
        }
    }

    fn garbage_count(&self) -> u32 {
        match self {
            GroupElement::Garbage(i) => *i,
            GroupElement::Group(v) => {
                let mut result = 0;
                for e in v {
                    result += e.garbage_count();
                }
                result
            }
        }
    }
}

fn parse_garbage_element(input: &str) -> IResult<&str, u32> {
    alt((
        value(1, none_of("!>")),
        value(0, preceded(parse_char('!'), anychar)),
    ))(input)
}

fn parse_garbage(input: &str) -> IResult<&str, GroupElement> {
    let garbage_parser = delimited(
        parse_char('<'),
        many0(parse_garbage_element),
        parse_char('>'),
    );
    map(garbage_parser, |v| GroupElement::Garbage(v.iter().sum()))(input)
}

fn parse_group_element(input: &str) -> IResult<&str, GroupElement> {
    let parse_element = alt((parse_garbage, parse_group_element));
    let group_vec_parser = delimited(
        parse_char('{'),
        separated_list(parse_char(','), parse_element),
        parse_char('}'),
    );
    map(group_vec_parser, |v| GroupElement::Group(v))(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let element = parse_group_element(input).unwrap().1;

    println!(
        "The total score for all groups in the input: {}",
        element.score(1)
    );
    println!(
        "The total garbage amount in the input: {}",
        element.garbage_count()
    );

    Ok(())
}
