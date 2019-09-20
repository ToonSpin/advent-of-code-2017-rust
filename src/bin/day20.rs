use std::io;
use std::io::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use std::collections::HashMap;
use std::ops::Sub;

type Coord = i64;

#[derive(Clone, Eq, PartialEq)]
struct Coords(Coord, Coord, Coord);

impl Sub for Coords {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Coords(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Clone)]
struct Particle {
    p: Coords,
    v: Coords,
    a: Coords,
}

impl Particle {
    fn position_at(&self, t: i64) -> Coords {
        Coords(
            self.p.0 + t * self.v.0 + t * (t + 1) * self.a.0 / 2,
            self.p.1 + t * self.v.1 + t * (t + 1) * self.a.1 / 2,
            self.p.2 + t * self.v.2 + t * (t + 1) * self.a.2 / 2,
        )
    }
    fn earliest_collision(&self, other: &Self) -> Option<i64> {
        let mut earliest_collision = None;

        let diff = self.clone() - other.clone();
        let candidates = discrete_zeros(diff.p.0, diff.v.0, diff.a.0);

        for t in candidates {
            if t >= 0 && self.position_at(t) == other.position_at(t) {
                match earliest_collision {
                    None => {
                        earliest_collision = Some(t);
                    }
                    Some(u) => {
                        if u > t {
                            earliest_collision = Some(t);
                        }
                    }
                }
            }
        }

        earliest_collision
    }
}

impl Sub for Particle {
    type Output = Particle;
    fn sub(self, other: Self) -> Self::Output {
        Particle {
            p: self.p - other.p,
            v: self.v - other.v,
            a: self.a - other.a,
        }
    }
}

fn is_perfect_square(n: i64) -> bool {
    if n < 0 {
        false
    } else {
        match n % 12 {
            0 | 1 | 4 | 9 => (n as f64).sqrt().fract() == 0.0,
            _ => false,
        }
    }
}

fn discrete_zeros(p: i64, v: i64, a: i64) -> Vec<i64> {
    if a == 0 {
        if v == 0 {
            if p == 0 {
                vec![0]
            } else {
                Vec::new()
            }
        } else {
            if p % v == 0 {
                vec![-p / v]
            } else {
                Vec::new()
            }
        }
    } else {
        let d_4 = a + 2 * v;
        let d_4 = d_4 * d_4 - 8 * a * p;

        if d_4 < 0 {
            Vec::new()
        } else if d_4 == 0 {
            if a % 2 == 1 || -(a / 2 + v) % a != 0 {
                Vec::new()
            } else {
                vec![-(a / 2 + v) / a]
            }
        } else if is_perfect_square(d_4) {
            let sd_2 = (d_4 as f64).sqrt().trunc() as i64;
            let mut s = Vec::new();
            if (-a - 2 * v + sd_2) % (2 * a) == 0 {
                s.push((-a - 2 * v + sd_2) / (2 * a));
            }
            if (-a - 2 * v - sd_2) % (2 * a) == 0 {
                s.push((-a - 2 * v - sd_2) / (2 * a));
            }
            s
        } else {
            Vec::new()
        }
    }
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let r = recognize(pair(opt(char('-')), digit1));
    map_res(r, str::parse::<Coord>)(input)
}

fn parse_coords(input: &str) -> IResult<&str, Coords> {
    let parser = tuple((
        parse_coord,
        preceded(char(','), parse_coord),
        preceded(char(','), parse_coord),
    ));
    let parser = map(parser, |c| Coords(c.0, c.1, c.2));
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

    let part1 = input
        .iter()
        .enumerate()
        .min_by_key(|(_i, p)| (p.a.0.abs() + p.a.1.abs() + p.a.2.abs()));
    println!(
        "The particle that will stay closest to the origin in the long run: {}",
        part1.unwrap().0
    );

    let mut earliest_collisions: Vec<Option<i64>> = vec![None; input.len()];
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some(t) = input[i].earliest_collision(&input[j]) {
                earliest_collisions[i] = match earliest_collisions[i] {
                    None => Some(t),
                    Some(u) => {
                        if u > t {
                            Some(t)
                        } else {
                            Some(u)
                        }
                    }
                };
                earliest_collisions[j] = match earliest_collisions[j] {
                    None => Some(t),
                    Some(u) => {
                        if u > t {
                            Some(t)
                        } else {
                            Some(u)
                        }
                    }
                };
            }
        }
    }

    let mut timeline: HashMap<i64, Vec<usize>> = HashMap::new();
    for (i, o) in earliest_collisions.iter().enumerate() {
        if let Some(t) = o {
            let v = timeline.entry(*t).or_insert(Vec::new());
            v.push(i);
        }
    }

    let mut count = input.len();
    for v in timeline.values() {
        if v.len() > 1 {
            count -= v.len();
        }
    }
    println!(
        "Number of particles left after all collisions have been resolved: {}",
        count
    );

    Ok(())
}
