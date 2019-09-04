use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Passphrase {
    words: Vec<[u8; 26]>,
}

impl Passphrase {
    fn is_valid(&self) -> bool {
        let l = self.words.len();
        for p in 0..l {
            for q in p + 1..l {
                if self.words[p] == self.words[q] {
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
        let mut words = Vec::new();
        for word in phrase.iter() {
            words.push(Self::get_signature(word));
        }
        Passphrase {
            words,
        }
    }
}

fn main() -> io::Result<()> {
    let mut input: Vec<String> = Vec::new();
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }
    let mut count = 0;
    for line in input.iter_mut() {
        if Passphrase::new(line).is_valid() {
            count += 1;
        }
    }
    println!("Number of valid passphrases in input: {:?}", count);
    Ok(())
}
