use std::io;
use std::io::prelude::*;

fn count_matches(input: &Vec<u64>, part2: bool) -> u64 {
    let multipliers: Vec<u64> = vec![16807, 48271];
    let modulus_per_generator: Vec<u64> = vec![4, 8];
    let modulus: u64 = 2147483647;
    let iteration_count = if part2 { 5_000_000 } else { 40_000_000 };

    let mut count = 0;
    let mut input = input.clone();

    for _iteration in 0..iteration_count {
        for i in 0..=1 {
            loop {
                input[i] *= multipliers[i];
                input[i] %= modulus;
                if !part2 || input[i] % modulus_per_generator[i] == 0 {
                    break;
                }
            }
        }
        if input[0] % 65536 == input[1] % 65536 {
            count += 1;
        }
    }
    count
}

fn main() -> io::Result<()> {
    let mut input: Vec<u64> = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = &line[24..];
        input.push(line.parse().unwrap());
    }

    println!("Final count (part 1): {}", count_matches(&input, false));
    println!("Final count (part 2): {}", count_matches(&input, true));

    Ok(())
}
