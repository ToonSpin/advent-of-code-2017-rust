use std::io;
use std::io::prelude::*;

struct Layer {
    depth: u32,
    range: u32
}

fn caught(l: &Layer, delay: u32) -> bool {
    return (l.depth + delay) % (2 * l.range - 2) == 0
}

fn trip_severity(input: &Vec<Layer>, delay: u32) -> u32 {
    input.iter().filter(|l| caught(l, delay)).fold(0, |a, e| a + e.depth * e.range)
}

fn trip_caught(input: &Vec<Layer>, delay: u32) -> bool {
    match input.iter().find(|l| caught(l, delay)) {
        Some(_) => true,
        None => false,
    }
}

fn main() -> io::Result<()> {
    let mut input = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut split = line.split(": ");

        let depth = split.next().unwrap().parse().unwrap();
        let range = split.next().unwrap().parse().unwrap();
        input.push(Layer { depth, range });
    }

    println!("The severity of the trip with delay 0: {}", trip_severity(&input, 0));

    let mut delay = 1;
    while trip_caught(&input, delay) {
        delay += 1;
    }

    println!("The smallest delay for which you don't get caught: {}", delay);

    Ok(())
}
