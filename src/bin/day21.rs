use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value, verify},
    IResult,
    multi::{count, separated_list},
    sequence::separated_pair
};

#[derive(Clone, Hash)]
struct Square {
    data: Vec<Vec<u8>>,
    size: usize
}

impl Square {
    fn new(data: Vec<Vec<u8>>) -> Square {
        let size = data[0].len();
        for v in data.iter() {
            assert!(v.len() == size);
        }
        Square {
            data,
            size
        }
    }

    fn sum(&self) -> u64 {
        self.data.iter().flatten().map(|n|*n as u64).sum()
    }

    fn compare_square_data(data1: &Vec<Vec<u8>>, data2: &Vec<Vec<u8>>, size: usize) -> bool {
        if data1 == data2 {
            return true;
        }

        if Self::compare_square_vertical_flip(data1, data2, size) {
            return true;
        }

        if Self::compare_square_horizontal_flip(data1, data2, size) {
            return true;
        }

        return false;
    }

    fn rotate_square_data(data: Vec<Vec<u8>>, size: usize) -> Vec<Vec<u8>> {
        let mut rotated: Vec<Vec<u8>> = vec![vec![0; size]; size];
        for y in 0..size {
            for x in 0..size {
                rotated[x][size - y - 1] = data[y][x];
            }
        }
        rotated
    }

    fn compare_square_horizontal_flip(data1: &Vec<Vec<u8>>, data2: &Vec<Vec<u8>>, size: usize) -> bool {
        for row in 0..size {
            for col in 0..size {
                if data1[row][col] != data2[row][size - col - 1] {
                    return false;
                }
            }
        }
        return true;
    }

    fn compare_square_vertical_flip(data1: &Vec<Vec<u8>>, data2: &Vec<Vec<u8>>, size: usize) -> bool {
        for row in 0..size {
            if data1[row] != data2[size - row - 1] {
                return false;
            }
        }
        true
    }

    fn split_squares(squares: Vec<Vec<Square>>) -> Vec<Vec<Square>> {
        let old_size = squares[0][0].size;
        let total_size = old_size * squares[0].len();
        let new_size = if total_size % 2 == 0 { 2 } else { 3 };
        let _num_squares_in_row = total_size / new_size;

        let new_square = Square { data: vec![vec![0; new_size]; new_size], size: new_size };
        let mut new_squares = vec![vec![new_square.clone(); total_size / new_size]; total_size / new_size];

        for total_row in 0..total_size {
            for total_col in 0..total_size {
                let new_sq_row = total_row / new_size;
                let new_sq_col = total_col / new_size;
                let old_sq_row = total_row / old_size;
                let old_sq_col = total_col / old_size;
                let value = squares[old_sq_row][old_sq_col].data[total_row % old_size][total_col % old_size];
                new_squares[new_sq_row][new_sq_col].data[total_row % new_size][total_col % new_size] = value;
            }
        }
        new_squares
    }
}

impl Eq for Square {
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        let other_data = other.data.clone();

        if Self::compare_square_data(&self.data, &other_data, self.size) {
            return true;
        }

        let other_data = Self::rotate_square_data(other_data, self.size);
        if Self::compare_square_data(&self.data, &other_data, self.size) {
            return true;
        }

        let other_data = Self::rotate_square_data(other_data, self.size);
        if Self::compare_square_data(&self.data, &other_data, self.size) {
            return true;
        }

        let other_data = Self::rotate_square_data(other_data, self.size);
        if Self::compare_square_data(&self.data, &other_data, self.size) {
            return true;
        }

        return false;
    }
}

fn parse_cell(input: &str) -> IResult<&str, u8> {
    alt((value(1, char('#')), value(0, char('.'))))(input)
}

fn parse_square(input: &str, n: usize) -> IResult<&str, Square> {
    let cell_parser = count(parse_cell, n);
    let square_parser = verify(separated_list(char('/'), cell_parser), |v: &Vec<_>| v.len() == n);
    map(square_parser, |v| Square::new(v))(input)
}

fn parse_square_2(input: &str) -> IResult<&str, Square> {
    parse_square(input, 2)
}

fn parse_square_3(input: &str) -> IResult<&str, Square> {
    parse_square(input, 3)
}

fn parse_square_4(input: &str) -> IResult<&str, Square> {
    parse_square(input, 4)
}

fn parse_mapping_2_3(input: &str) -> IResult<&str, (Square, Square)> {
    separated_pair(parse_square_2, tag(" => "), parse_square_3)(input)
}

fn parse_mapping_3_4(input: &str) -> IResult<&str, (Square, Square)> {
    separated_pair(parse_square_3, tag(" => "), parse_square_4)(input)
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<(Square, Square)>> {
    separated_list(char('\n'), alt((parse_mapping_2_3, parse_mapping_3_4)))(input)
}

fn iterate(iterations: u32, squares: &Vec<Vec<Square>>, mappings: &Vec<(Square, Square)>) -> Vec<Vec<Square>> {
    let mut squares = squares.clone();
    for _iteration in 0..iterations {
        let mut new_set = Vec::new();

        for r in squares.iter() {
            let mut new_row = Vec::new();
            for s in r.iter() {
                for (source, dest) in mappings.iter() {
                    if *source == *s {
                        new_row.push(dest.clone());
                        break;
                    }
                }
            }
            new_set.push(new_row);
        }

        squares = Square::split_squares(new_set)
    }
    squares
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, mappings) = parse_mappings(input).unwrap();

    let (_dummy, start_square) = parse_square_3(".#./..#/###").unwrap();

    let current_set = vec![vec![start_square.clone()]];
    println!("Sum after 5 iterations: {}", iterate(5, &current_set, &mappings).iter().flatten().fold(0, |a, e| a + e.sum()));

    let mut cache: HashMap<Square, Vec<Square>> = HashMap::new();
    let mut current_set = vec![start_square];
    for _i in 1..=6 {
        let mut new_set = Vec::new();
        for square in current_set.iter() {
            if cache.contains_key(square) {
                for s in cache.get(square).unwrap().iter() {
                    new_set.push(s.clone());
                }
            } else {
                let result = iterate(3, &vec![vec![square.clone()]], &mappings);
                for s in result.iter().flatten() {
                    new_set.push(s.clone());
                }
                cache.insert(square.clone(), result.iter().flatten().map(|s| s.clone()).collect());
            }
        }
        current_set = new_set;
    }
    println!("Sum after 18 iterations: {}", current_set.iter().fold(0, |a, e| a + e.sum()));

    Ok(())
}
