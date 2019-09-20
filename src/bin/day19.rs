use std::io;
use std::io::prelude::*;

use nom::character::is_alphabetic;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];
    let input: Vec<Vec<char>> = input
        .split("\n")
        .filter(|s| s.len() > 1)
        .map(|s| s.chars().collect())
        .collect();

    let mut coords = (0, 0);
    let mut dir = Direction::South;
    let mut letters = Vec::new();
    let mut step_count = 0;

    let directions: Vec<(i32, i32, Direction)> = vec![
        (0, -1, Direction::North),
        (1, 0, Direction::East),
        (0, 1, Direction::South),
        (-1, 0, Direction::West),
    ];

    for (i, c) in input[0].iter().enumerate() {
        if *c != ' ' {
            coords = (i, 0);
            break;
        }
    }

    while letters.len() < 10 {
        match input[coords.1][coords.0] {
            '+' => {
                for (e, n, d) in directions.iter() {
                    let x = (coords.0 as i32 + e) as usize;
                    let y = (coords.1 as i32 + n) as usize;

                    if dir != d.opposite() && input[y][x] != ' ' {
                        dir = *d;
                        break;
                    }
                }
            }
            ' ' => unreachable!(),
            c if is_alphabetic(c as u8) => letters.push(c),
            _ => {}
        }

        match dir {
            Direction::North => {
                coords.1 -= 1;
            }
            Direction::East => {
                coords.0 += 1;
            }
            Direction::South => {
                coords.1 += 1;
            }
            Direction::West => {
                coords.0 -= 1;
            }
        }
        step_count += 1;
    }

    print!("The letters as encountered by the packet: ");
    for c in letters.iter() {
        print!("{}", c);
    }
    println!("");

    println!("Total number of steps: {}", step_count);

    Ok(())
}
