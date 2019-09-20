use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Passphrase<'a> {
    phrase: Vec<&'a str>,
    signatures: Vec<[u8; 26]>,
}

impl<'a> Passphrase<'a> {
    fn is_valid_part1(&self) -> bool {
        let l = self.phrase.len();
        for p in 0..l {
            for q in p + 1..l {
                if self.phrase[p] == self.phrase[q] {
                    return false;
                }
            }
        }
        true
    }

    fn is_valid_part2(&self) -> bool {
        let l = self.signatures.len();
        for p in 0..l {
            for q in p + 1..l {
                if self.signatures[p] == self.signatures[q] {
                    return false;
                }
            }
        }
        true
    }

    fn get_signature(s: &str) -> [u8; 26] {
        let mut a = [0; 26];
        for b in s.as_bytes().iter() {
            a[(*b - b'a') as usize] += 1;
        }
        a
    }

    fn new(s: &str) -> Passphrase {
        let phrase: Vec<&str> = s.split(' ').collect();
        let mut signatures = Vec::new();
        for word in phrase.iter() {
            signatures.push(Self::get_signature(word));
        }
        Passphrase { phrase, signatures }
    }
}

fn main() -> io::Result<()> {
    let mut input: Vec<String> = Vec::new();
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    let passphrases: Vec<Passphrase> = input.iter_mut().map(|line| Passphrase::new(line)).collect();

    println!(
        "Number of valid passphrases in input (part 1): {}",
        passphrases.iter().filter(|p| p.is_valid_part1()).count()
    );
    println!(
        "Number of valid passphrases in input (part 2): {}",
        passphrases.iter().filter(|p| p.is_valid_part2()).count()
    );

    Ok(())
}
