use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res, opt, recognize},
    IResult,
    multi::separated_list,
    sequence::{pair, terminated},
};

enum ModificationOperator {
    Inc,
    Dec
}
use ModificationOperator::*;

enum ComparisonOperator {
    Eq,
    Neq,
    Gt,
    Lt,
    Ge,
    Le,
}
use ComparisonOperator::*;

struct Instruction<'a> {
    register: &'a str,
    mod_op: ModificationOperator,
    mod_val: i32,
    cmp_register: &'a str,
    cmp_op: ComparisonOperator,
    cmp_val: i32,
}

impl<'a> Instruction<'a> {
    fn test(&self, registers: &HashMap<&str, i32>) -> bool {
        let reg_val = match registers.get(self.cmp_register) {
            Some(&v) => v,
            None => 0,
        };
        match self.cmp_op {
            Eq  => reg_val == self.cmp_val,
            Neq => reg_val != self.cmp_val,
            Gt  => reg_val >  self.cmp_val,
            Lt  => reg_val <  self.cmp_val,
            Ge  => reg_val >= self.cmp_val,
            Le  => reg_val <= self.cmp_val,
        }
    }
}

fn parse_mod_op(input: &str) -> IResult<&str, ModificationOperator> {
    let f = |s| match s {
        "inc" => Inc,
        "dec" => Dec,
        _ => unreachable!()
    };
    map(alt((tag("inc"), tag("dec"))), f)(input)
}

fn parse_cmp_op(input: &str) -> IResult<&str, ComparisonOperator> {
    let f = |s| match s {
        "==" => Eq,
        "!=" => Neq,
        ">"  => Gt,
        "<"  => Lt,
        ">=" => Ge,
        "<=" => Le,
        _ => unreachable!()
    };
    map(alt((tag("=="), tag("!="), tag(">="), tag("<="), tag(">"), tag("<"))), f)(input)
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let r = recognize(pair(opt(tag("-")), digit1));
    map_res(r, str::parse::<i32>)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    // oui inc 550 if p <= 3
    let (input, register) = terminated(alpha1, tag(" "))(input)?;
    let (input, mod_op) = terminated(parse_mod_op, tag(" "))(input)?;
    let (input, mod_val) = terminated(parse_i32, tag(" if "))(input)?;
    let (input, cmp_register) = terminated(alpha1, tag(" "))(input)?;
    let (input, cmp_op) = terminated(parse_cmp_op, tag(" "))(input)?;
    let (input, cmp_val) = parse_i32(input)?;

    Ok((input, Instruction {
        register,
        mod_op,
        mod_val,
        cmp_register,
        cmp_op,
        cmp_val
    }))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list(tag("\n"), parse_instruction)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, instructions) = parse_instructions(input).unwrap();
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut max_reg_val = 0;

    for i in instructions {
        if i.test(&registers) {
            let reg_val = registers.entry(i.register).or_insert(0);
            match i.mod_op {
                Inc => {
                    *reg_val += i.mod_val;
                }
                Dec => {
                    *reg_val -= i.mod_val;
                }
            }
            if max_reg_val < *reg_val {
                max_reg_val = *reg_val;
            }
        }
    }

    println!("The maximum value of any register after visiting all instructions: {}", registers.values().max().unwrap());
    println!("The maximum value of any register at any time: {}", max_reg_val);

    Ok(())
}
