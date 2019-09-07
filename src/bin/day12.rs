use std::io;
use std::io::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{char as parse_char, digit1},
    combinator::map_res,
    IResult,
    multi::separated_list,
    sequence::separated_pair,
};

use std::collections::HashSet;

struct Node {
    pipes: Vec<u32>,
    visited: bool
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn parse_node_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list(tag(", "), parse_u32)(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (rest, (_id, pipes)) = separated_pair(parse_u32, tag(" <-> "), parse_node_list)(input)?;
    Ok((rest, Node { pipes, visited: false }))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    separated_list(parse_char('\n'), parse_node)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let (_rest, mut input) = parse_nodes(input).unwrap();

    let mut part1_done: bool = false;
    let mut group_count: u32 = 0;

    loop {
        let mut group: HashSet<u32> = HashSet::new();
        let mut queue: Vec<u32> = Vec::new();

        for (i, n) in input.iter().enumerate() {
            if !n.visited {
                queue.push(i as u32);
                break;
            }
        }

        if queue.len() == 0 {
            break;
        }

        while queue.len() > 0 {
            let current_element = queue.pop().unwrap();
            if group.contains(&current_element) {
                continue;
            }

            group.insert(current_element);
            let mut node = &mut input[current_element as usize];
            node.visited = true;
            for connected_node in node.pipes.iter() {
                queue.push(*connected_node);
            }
        }

        group_count += 1;

        if !part1_done {
            println!("The number of programs in the group that contains program 0: {}", group.len());
            part1_done = true;
        }
    }

    println!("The number of groups in the input: {}", group_count);

    Ok(())
}
