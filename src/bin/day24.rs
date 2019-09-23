use std::io;
use std::io::prelude::*;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list,
    sequence::separated_pair,
    IResult,
};

type Port = u8;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Component {
    p0: Port,
    p1: Port,
    strength: u32,
    length: u32,
}

impl Component {
    fn other_port(&self, p: Port) -> Port {
        if self.p0 == p {
            self.p1
        } else {
            self.p0
        }
    }

    fn next_port(v: &Vec<Component>) -> Port {
        let mut next_port = v[0].other_port(0);
        for i in 1..v.len() {
            next_port = v[i].other_port(next_port);
        }
        next_port
    }

    fn can_be_added_to(&self, v: &Vec<Component>) -> bool {
        if v.len() == 0 {
            true
        } else {
            let next_port = Self::next_port(v);
            self.p0 == next_port || self.p1 == next_port
        }
    }

    fn join(a: &Component, b: &Component, common_port: Port) -> Component {
        let p0 = a.other_port(common_port);
        let p1 = b.other_port(common_port);
        let strength = a.strength + b.strength;
        let length = a.length + b.length;
        Component {
            p0,
            p1,
            strength,
            length,
        }
    }
}

fn parse_port(input: &str) -> IResult<&str, Port> {
    map_res(digit1, str::parse::<Port>)(input)
}

fn parse_component(input: &str) -> IResult<&str, Component> {
    map(
        separated_pair(parse_port, char('/'), parse_port),
        |(p0, p1)| Component {
            p0,
            p1,
            strength: (p0 + p1) as u32,
            length: 1,
        },
    )(input)
}

fn parse_components(input: &str) -> IResult<&str, Vec<Component>> {
    separated_list(char('\n'), parse_component)(input)
}

fn strength_of_strongest_bridge(base: &mut Vec<Component>, input: &mut HashSet<Component>) -> u32 {
    let strength = base.iter().map(|c| c.strength).sum();
    if input.len() == 0 {
        return strength;
    }

    let candidates: Vec<Component> = input
        .iter()
        .filter(|c| c.can_be_added_to(&base))
        .cloned()
        .collect();
    if candidates.len() == 0 {
        return strength;
    }

    let mut max_strength = strength;
    for c in candidates.iter() {
        base.push(*c);
        input.remove(&c);
        let strength = strength_of_strongest_bridge(base, input);
        if strength > max_strength {
            max_strength = strength;
        }
        input.insert(base.pop().unwrap());
    }
    max_strength
}

fn compare_bridges(a: &&(u32, u32), b: &&(u32, u32)) -> Ordering {
    a.0.cmp(&b.0).then(a.1.cmp(&b.1))
}

fn get_longest_bridge(base: &mut Vec<Component>, input: &mut HashSet<Component>) -> (u32, u32) {
    let strength = base.iter().map(|c| c.strength).sum();
    let length = base.iter().map(|c| c.length).sum();
    if input.len() == 0 {
        return (length, strength);
    }

    let candidates: Vec<Component> = input
        .iter()
        .filter(|c| c.can_be_added_to(&base))
        .cloned()
        .collect();
    if candidates.len() == 0 {
        return (length, strength);
    }

    let mut longest_bridge = (length, strength);
    for c in candidates.iter() {
        base.push(*c);
        input.remove(&c);
        let bridge = get_longest_bridge(base, input);
        if let Ordering::Greater = compare_bridges(&&bridge, &&longest_bridge) {
            longest_bridge = bridge;
        }
        input.insert(base.pop().unwrap());
    }
    longest_bridge
}

fn starting_points(input: &Vec<Component>) -> Vec<Component> {
    input
        .iter()
        .cloned()
        .filter(|c| c.p0 == 0 || c.p1 == 0)
        .collect()
}

fn reduce_chains(input: &Vec<Component>) -> Vec<Component> {
    let mut ports = HashMap::new();
    for c in input.iter() {
        *ports.entry(c.p0).or_insert(0) += 1;
        *ports.entry(c.p1).or_insert(0) += 1;
    }
    let ports_iter = ports
        .iter()
        .filter(|(&p, &n)| p != 0 && n == 2)
        .map(|(&p, &_n)| p);
    let ports: HashSet<Port> = HashSet::from_iter(ports_iter);

    let mut output = input.clone();
    for p in ports.iter() {
        output = Vec::new();
        let matches: Vec<Component> = input
            .iter()
            .filter(|&c| c.p0 == *p || c.p1 == *p)
            .cloned()
            .collect();
        output.push(Component::join(&matches[0], &matches[1], *p));
        for c in input.iter().filter(|&c| c.p0 != *p && c.p1 != *p) {
            output.push(*c);
        }
    }
    if ports.len() > 0 {
        reduce_chains(&output)
    } else {
        output
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = parse_components(input).unwrap();

    let input = reduce_chains(&input);

    let mut max_strength = 0;
    let mut longest_bridges = Vec::new();
    let mut input_set = HashSet::from_iter(input.iter().cloned());
    for p in starting_points(&input).iter() {
        input_set.remove(p);
        let strength = strength_of_strongest_bridge(&mut vec![*p], &mut input_set);
        if strength > max_strength {
            max_strength = strength;
        }
        longest_bridges.push(get_longest_bridge(&mut vec![*p], &mut input_set));
        input_set.insert(*p);
    }

    let best_bridge = longest_bridges.iter().max_by(compare_bridges);

    println!(
        "The strength of the strongest bridge that can be built: {}",
        max_strength
    );
    println!(
        "The strength of the longest bridge that can be built: {}",
        best_bridge.unwrap().1
    );

    Ok(())
}
