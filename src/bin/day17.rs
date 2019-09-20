use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: usize = input[..].parse().unwrap();

    let mut buffer: Vec<u32> = Vec::with_capacity(2018);
    buffer.push(0);

    let mut current_pos: usize = 0;

    for i in 1..=2017 {
        current_pos += input;
        current_pos %= i;
        current_pos += 1;
        buffer.insert(current_pos, i as u32);
    }

    println!(
        "The value after 2017 is: {}",
        buffer[(current_pos + 1) % buffer.len()]
    );

    current_pos = 0;
    let mut current_after_0 = 0;

    for i in 1..=50_000_000 {
        current_pos += input;
        current_pos %= i;
        current_pos += 1;
        if current_pos == 1 {
            current_after_0 = i;
        }
    }

    println!(
        "The value after 0 after 50 million iterations is: {}",
        current_after_0
    );
    Ok(())
}
