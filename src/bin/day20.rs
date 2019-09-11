use std::io;
use std::io::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize},
    IResult,
    multi::separated_list,
    sequence::{delimited, pair, preceded, tuple},
};

type Coord = i64;
type Coords = (Coord, Coord, Coord);

struct Particle {
    p: Coords,
    v: Coords,
    a: Coords
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let r = recognize(pair(opt(char('-')), digit1));
    map_res(r, str::parse::<Coord>)(input)
}

fn parse_coords(input: &str) -> IResult<&str, Coords> {
    let parser = tuple((parse_coord, preceded(char(','), parse_coord), preceded(char(','), parse_coord)));
    delimited(char('<'), parser, char('>'))(input)
}

fn parse_particle(input: &str) -> IResult<&str, Particle> {
    let parser = tuple((
        preceded(tag("p="), parse_coords),
        preceded(tag(", v="), parse_coords),
        preceded(tag(", a="), parse_coords),
    ));
    map(parser, |(p, v, a)| Particle { p, v, a })(input)
}

fn parse_particles(input: &str) -> IResult<&str, Vec<Particle>> {
    separated_list(char('\n'), parse_particle)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = parse_particles(input).unwrap();

    let part1 = input.iter().enumerate().min_by_key(|(_i, p)| (p.a.0.abs() +  p.a.1.abs() +  p.a.2.abs()));
    println!("The particle that will stay closest to the origin in the long run: {}", part1.unwrap().0);

    Ok(())
}
