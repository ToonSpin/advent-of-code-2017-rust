use std::io;
use std::io::prelude::*;

fn reverse_section(mut v: Vec<u8>, section_start: usize, section_length: usize) -> Vec<u8> {
    let max: usize = section_length / 2;
    let len = v.len();

    for i in 0..max {
        let p = section_start + i;
        let q = section_start + section_length - 1 - i;

        let temp = v[p % len];
        v[p % len] = v[q % len];
        v[q % len] = temp;
    }

    v
}

fn hash_rounds(input: &Vec<u8>, rounds: u8) -> Vec<u8> {
    let mut skip_size = 0;
    let mut start = 0;
    let mut numbers = vec![];

    for i in 0..=255 {
        numbers.push(i);
    }

    for _round in 0..rounds {
        for i in input.iter() {
            numbers = reverse_section(numbers, start, *i as usize);
            start += *i as usize + skip_size;
            start %= 256;
            skip_size += 1;
        }
    }

    numbers
}

fn get_dense_hash(input: &str) -> Vec<u8> {
    let mut input: Vec<u8> = input.as_bytes().iter().map(|b| *b as u8).collect();
    input.append(&mut vec![17, 31, 73, 47, 23]);

    let mut result = Vec::new();
    let sparse_hash = hash_rounds(&input, 64);
    for c in sparse_hash.chunks(16) {
        result.push(c.iter().fold(0, |a, e| a ^ e));
    }
    result
}

fn knot_hash_ones(input: &str) -> Vec<u8> {
    let v = get_dense_hash(input);
    let mut ones = Vec::new();
    for i in v.iter() {
        let mut i = *i;
        for _j in 0..8 {
            ones.push(i / 128);
            i %= 128;
            i *= 2;
        }
    }
    ones
}

fn get_neighbors(x: u8, y: u8) -> Vec<(u8, u8)> {
    let mut v = Vec::new();
    let neighbors: [(i16, i16); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (p, q) in &neighbors {
        if x == 127 && *p > 0 {
            continue;
        }
        if y == 127 && *q > 0 {
            continue;
        }
        if x == 0 && *p < 0 {
            continue;
        }
        if y == 0 && *q < 0 {
            continue;
        }
        v.push(((*p + x as i16) as u8, (*q + y as i16) as u8));
    }
    v
}

fn remove_region_from_grid(mut grid: Vec<Vec<u8>>, x: u8, y: u8) -> Vec<Vec<u8>> {
    let mut queue = vec![(x, y)];
    while queue.len() > 0 {
        let (p, q) = queue.pop().unwrap();
        grid[q as usize][p as usize] = 0;
        for (r, s) in get_neighbors(p, q) {
            if grid[s as usize][r as usize] > 0 {
                queue.push((r, s))
            }
        }
    }
    grid
}

fn find_region_in_grid(grid: &Vec<Vec<u8>>) -> Option<(u8, u8)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, _cell) in row.iter().enumerate() {
            if grid[y][x] > 0 {
                return Some((x as u8, y as u8));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let mut total_count = 0u32;
    let mut grid = vec![];
    for i in 0..128 {
        let hash_input = &format!("{}-{}", input, i)[..];
        let ones = knot_hash_ones(hash_input);
        total_count += ones.iter().sum::<u8>() as u32;
        grid.push(ones);
    }

    println!("Number of squares used: {}", total_count);

    let mut num_regions = 0;
    while let Some((x, y)) = find_region_in_grid(&grid) {
        grid = remove_region_from_grid(grid, x, y);
        num_regions += 1;
    }

    println!("Number of regions in grid: {}", num_regions);

    Ok(())
}
