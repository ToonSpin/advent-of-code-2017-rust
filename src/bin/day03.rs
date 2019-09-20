use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

fn value_to_write(cells: &HashMap<(i32, i32), u32>, coords: (i32, i32)) -> u32 {
    let mut v = 0;
    for p in -1..=1 {
        for q in -1..=1 {
            if p != 0 || q != 0 {
                v += cells.get(&(coords.0 + p, coords.1 + q)).unwrap_or(&0);
            }
        }
    }
    v
}

fn max_for_tier(n: i32) -> i32 {
    let n = 2 * n + 1;
    n * n
}

fn tier(n: i32) -> i32 {
    (n as f32 * 0.25).sqrt().ceil() as i32
}

fn distance_from_origin(n: i32) -> i32 {
    let tier = tier(n);
    let n = n - max_for_tier(tier - 1);
    (n % (tier * 2) - tier).abs() + tier
}

fn main() -> io::Result<()> {
    let mut input: u32 = 0;

    let mut cells: HashMap<(i32, i32), u32> = HashMap::new();
    cells.insert((0, 0), 1);

    for line in io::stdin().lock().lines() {
        input = line.unwrap().parse().unwrap();
    }

    println!("Steps required to carry the data to the access port: {}", distance_from_origin(input as i32));

    let mut current_coords = (1, 0);
    let mut current_value;
    loop {
        while cells.contains_key(&(current_coords.0 - 1, current_coords.1)) {
            current_value = value_to_write(&cells, current_coords);
            if current_value > input {
                break;
            }
            cells.insert(current_coords, current_value);
            current_coords.1 -= 1;
        }

        current_value = value_to_write(&cells, current_coords);
        if current_value > input {
            break;
        }
        cells.insert(current_coords, current_value);
        current_coords.0 -= 1;

        while cells.contains_key(&(current_coords.0, current_coords.1 + 1)) {
            current_value = value_to_write(&cells, current_coords);
            if current_value > input {
                break;
            }
            cells.insert(current_coords, current_value);
            current_coords.0 -= 1;
        }

        current_value = value_to_write(&cells, current_coords);
        if current_value > input {
            break;
        }
        cells.insert(current_coords, current_value);
        current_coords.1 += 1;

        while cells.contains_key(&(current_coords.0 + 1, current_coords.1)) {
            current_value = value_to_write(&cells, current_coords);
            if current_value > input {
                break;
            }
            cells.insert(current_coords, current_value);
            current_coords.1 += 1;
        }

        current_value = value_to_write(&cells, current_coords);
        if current_value > input {
            break;
        }
        cells.insert(current_coords, current_value);
        current_coords.0 += 1;

        while cells.contains_key(&(current_coords.0, current_coords.1 - 1)) {
            current_value = value_to_write(&cells, current_coords);
            if current_value > input {
                break;
            }
            cells.insert(current_coords, current_value);
            current_coords.0 += 1;
        }

        while current_coords.1 != 0 {
            current_value = value_to_write(&cells, current_coords);
            if current_value > input {
                break;
            }
            cells.insert(current_coords, current_value);
            current_coords.1 -= 1;
        }
    }

    println!("The first value written that is larger than the input: {}", current_value);

    Ok(())
}
