use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

type MemoryBankSet = [u32; 16];

fn redistribute_memory(input: &MemoryBankSet) -> MemoryBankSet {
    let mut new_memory_bank_set = input.clone();
    let mut bank_index = 0;
    let mut bank_count = 0;

    for i in 0..16 {
        if i == 0 || input[i] > bank_count {
            bank_count = input[i];
            bank_index = i;
        }
    }

    new_memory_bank_set[bank_index] = 0;
    bank_index += 1;
    bank_index %= 16;

    while bank_count > 0 {
        new_memory_bank_set[bank_index] += 1;
        bank_count -= 1;
        bank_index += 1;
        bank_index %= 16;
    }

    new_memory_bank_set
}

fn main() -> io::Result<()> {
    let mut input: MemoryBankSet = [0; 16];
    let mut states_found: HashSet<MemoryBankSet> = HashSet::new();
    states_found.insert(input);

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        for (i, number) in line.split('\t').enumerate() {
            input[i] = number.parse().unwrap();
        }
    }

    let mut num_cycles = 0;
    let mut part1_done = false;
    loop {
        num_cycles += 1;
        input = redistribute_memory(&input);
        if states_found.insert(input) == false {
            if part1_done {
                break;
            }

            println!(
                "Number of cycles before start of infinite loop: {}",
                num_cycles
            );
            states_found = HashSet::new();
            num_cycles = 0;
            part1_done = true;
        }
    }
    println!("Number of cycles in infinite loop: {}", num_cycles - 1);

    Ok(())
}
