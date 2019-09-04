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

fn main() -> io::Result<()> {
    let mut input: u32 = 0;

    let mut cells: HashMap<(i32, i32), u32> = HashMap::new();
    cells.insert((0, 0), 1);

    for line in io::stdin().lock().lines() {
        input = line.unwrap().parse().unwrap();
    }

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

    println!("{:?}", current_value);

    Ok(())
}
