use std::io;
use std::io::prelude::*;

fn solve_captcha(input: &Vec<u8>, offset: usize) -> u32 {
    let mut solution: u32 = 0;
    for i in 0..input.len() {
        if input[i] == input[(i + offset) % input.len()] {
            solution += input[i] as u32;
        }
    }
    solution
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<u8> = input.bytes().map(|b| b - b'0').collect();

    println!("The solution to the first captcha is: {}", solve_captcha(&input, 1));
    println!("The solution to the second captcha is: {}", solve_captcha(&input, input.len() / 2));

    Ok(())
}
