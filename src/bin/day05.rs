use std::io;
use std::io::prelude::*;

fn get_number_of_steps(input: &Vec<i32>, part2: bool) -> u32 {
    let mut input = input.clone();
    let mut pointer: usize = 0;
    let mut count: u32 = 0;

    while pointer < input.len() {
        let increment = if part2 && input[pointer] >= 3 { -1 } else { 1 };
        input[pointer] += increment;
        pointer = (pointer as i32 + input[pointer] - increment) as usize;
        count += 1;
    }

    count
}

fn main() -> io::Result<()> {
    let mut input: Vec<i32> = Vec::new();

    for line in io::stdin().lock().lines() {
        input.push(line.unwrap().parse().unwrap());
    }

    println!("Number of steps to reach the exit (part 1): {:?}", get_number_of_steps(&input, false));
    println!("Number of steps to reach the exit (part 2): {:?}", get_number_of_steps(&input, true));

    Ok(())
}
