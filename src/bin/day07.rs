use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map_res, opt},
    multi::separated_list,
    sequence::{preceded, terminated},
    IResult,
};

struct Program<'a> {
    name: &'a str,
    weight: i32,
    children: Vec<&'a str>,
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    return alpha1(input);
}

fn parse_weight(input: &str) -> IResult<&str, i32> {
    return map_res(digit1, str::parse::<i32>)(input);
}

fn parse_name_list(input: &str) -> IResult<&str, Vec<&str>> {
    let name_list_parser = separated_list(tag(", "), parse_name);
    let name_list_parser = opt(preceded(tag(" -> "), name_list_parser));
    let (rest, result) = name_list_parser(input).unwrap();
    let result = match result {
        None => Vec::new(),
        Some(r) => r,
    };
    Ok((rest, result))
}

fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, name) = terminated(parse_name, tag(" ("))(input)?;
    let (input, weight) = terminated(parse_weight, tag(")"))(input)?;
    let (input, children) = parse_name_list(input)?;

    Ok((
        input,
        Program {
            name,
            weight,
            children,
        },
    ))
}

fn get_combined_weight(p: &str, programs: &HashMap<&str, Program>) -> i32 {
    let p = programs.get(p).unwrap();
    let mut weight = p.weight;
    for c in p.children.iter() {
        weight += get_combined_weight(c, programs);
    }
    weight
}

fn get_unbalanced_child<'a>(
    p: &'a str,
    programs: &'a HashMap<&'a str, Program>,
) -> Option<(&'a str, i32)> {
    let mut found_weights: HashMap<i32, &str> = HashMap::new();
    let mut balanced_weight: i32 = 0;

    for c in programs.get(p).unwrap().children.iter() {
        let weight = get_combined_weight(c, programs);
        if found_weights.contains_key(&weight) {
            balanced_weight = weight;
        } else {
            if balanced_weight > 0 && balanced_weight != weight {
                return Some((c, balanced_weight - weight));
            }
        }
        found_weights.insert(weight, c);
    }

    if balanced_weight == 0 {
        return None;
    }

    for (w, c) in found_weights {
        if w != balanced_weight {
            return Some((c, balanced_weight - w));
        }
    }

    None
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let mut programs = HashMap::new();
    let mut has_parent = HashSet::new();
    for line in input.split("\n") {
        if line == "" {
            continue;
        }
        let p = parse_program(line).unwrap().1;
        programs.insert(p.name, p);
    }

    for (_n, p) in programs.iter() {
        for child in p.children.iter() {
            has_parent.insert(child);
        }
    }

    let mut current_program = "";
    for (n, _p) in programs.iter() {
        if !has_parent.contains(n) {
            current_program = n;
            break;
        }
    }

    println!("The bottom program is: {}", current_program);

    let mut difference = 0;
    while let Some((c, wd)) = get_unbalanced_child(current_program, &programs) {
        current_program = c;
        difference = wd;
    }

    println!(
        "To balance the programs, program {} needs to weigh {}",
        current_program,
        programs.get(current_program).unwrap().weight + difference
    );

    Ok(())
}
