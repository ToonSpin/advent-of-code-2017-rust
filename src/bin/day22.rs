use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::value,
    multi::{many1, separated_list},
    IResult,
};

enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn turn_left(&self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    fn reverse(&self) -> Direction {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
    fn move_forward(&self, coords: &(i32, i32)) -> (i32, i32) {
        match self {
            North => (coords.0, coords.1 - 1),
            East => (coords.0 + 1, coords.1),
            South => (coords.0, coords.1 + 1),
            West => (coords.0 - 1, coords.1),
        }
    }
}

#[derive(Clone, Copy)]
enum CellState {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

fn parse_cell_state(input: &str) -> IResult<&str, CellState> {
    alt((
        value(CellState::Infected, char('#')),
        value(CellState::Clean, char('.')),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<CellState>>> {
    separated_list(char('\n'), many1(parse_cell_state))(input)
}

fn prepare_nodes(input: &Vec<Vec<CellState>>) -> HashMap<(i32, i32), CellState> {
    let mut nodes: HashMap<(i32, i32), CellState> = HashMap::new();
    for (y, v) in input.iter().enumerate() {
        for (x, c) in v.iter().enumerate() {
            if let CellState::Infected = c {
                nodes.insert((x as i32, y as i32), CellState::Infected);
            }
        }
    }
    nodes
}

fn iterate(num_bursts: u64, input: &Vec<Vec<CellState>>, part2: bool) -> u64 {
    let current_coords = input[0].len() as i32 / 2;
    let mut current_coords = (current_coords, current_coords);
    let mut current_dir = North;

    let mut nodes = prepare_nodes(&input);

    let mut infections_count = 0;

    let clean_transition_state = if part2 {
        CellState::Weakened
    } else {
        CellState::Infected
    };
    let infected_transition_state = if part2 {
        CellState::Flagged
    } else {
        CellState::Clean
    };

    for _iteration in 0..num_bursts {
        match nodes.entry(current_coords).or_insert(CellState::Clean) {
            CellState::Clean => {
                current_dir = current_dir.turn_left();
                nodes.insert(current_coords, clean_transition_state);
                if !part2 {
                    infections_count += 1;
                }
            }
            CellState::Infected => {
                current_dir = current_dir.turn_right();
                nodes.insert(current_coords, infected_transition_state);
            }
            CellState::Weakened => {
                nodes.insert(current_coords, CellState::Infected);
                infections_count += 1;
            }
            CellState::Flagged => {
                current_dir = current_dir.reverse();
                nodes.insert(current_coords, CellState::Clean);
            }
        }
        current_coords = current_dir.move_forward(&current_coords);
    }
    infections_count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = parse_input(input).unwrap();

    println!(
        "Number of infections after 10000 iterations: {}",
        iterate(10000, &input, false)
    );
    println!(
        "Number of infections after 10000000 iterations: {}",
        iterate(10_000_000, &input, true)
    );

    Ok(())
}
