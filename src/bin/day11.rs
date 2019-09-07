use std::io;
use std::io::prelude::*;

enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Direction {
    fn new(input: &str) -> Direction {
        match input {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => unreachable!()
        }
    }

    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, 1),
            Direction::NE => (1, 1),
            Direction::SE => (1, 0),
            Direction::S => (0, -1),
            Direction::SW => (-1, -1),
            Direction::NW => (-1, 0),
        }
    }
}

fn shortest_path_length(offset: (i32, i32)) -> i32 {
    let (mut p, mut q) = offset;
    if q < 0 {
        p = -p;
        q = -q;
    }

    if p <= 0 {
        -p + q
    } else if p <= q {
        q
    } else {
        p
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let input: Vec<Direction> = input.split(',').map(|s| Direction::new(s)).collect();

    let mut x = 0;
    let mut y = 0;
    let mut longest_path_found = 0;

    for d in input.iter() {
        let (p, q) = d.get_offset();
        x += p;
        y += q;
        let path_length = shortest_path_length((x, y));
        if path_length > longest_path_found {
            longest_path_found = path_length;
        }
    }

    println!("The shortest path to the child process has length: {}", shortest_path_length((x, y)));
    println!("The farthest the child has ever been away from the parent is: {}", longest_path_found);

    Ok(())
}
