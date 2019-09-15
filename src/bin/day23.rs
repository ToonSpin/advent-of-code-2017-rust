use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::{map, map_res, opt, recognize},
    IResult,
    multi::separated_list,
    sequence::{pair, preceded, separated_pair, tuple},
};

type Register = char;
type Number = i64;

#[derive(Clone)]
enum Value {
    Register(Register),
    Literal(Number),
}

impl Value {
    fn resolve(&self, registers: &HashMap<Register, Number>) -> Number {
        match self {
            Value::Register(r) => {
                *registers.get(r).unwrap_or(&0)
            },
            Value::Literal(n) => *n
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Set(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Jnz(Value, Value),
}

fn parse_literal(input: &str) -> IResult<&str, Number> {
    let r = recognize(pair(opt(tag("-")), digit1));
    map_res(r, str::parse::<Number>)(input)
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    one_of("abcdefgh")(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let parse_register_as_value = map(parse_register, |r| Value::Register(r));
    let parse_literal_as_value = map(parse_literal, |n| Value::Literal(n));
    alt((parse_register_as_value, parse_literal_as_value))(input)
}

fn parse_instruction_val_val(input: &str) -> IResult<&str, Instruction> {
    let parse_instr = preceded(tag("jnz "), separated_pair(parse_value, tag(" "), parse_value));
    let (rest, (v1, v2)) = parse_instr(input)?;
    Ok((rest, Instruction::Jnz(v1, v2)))
}

fn parse_instruction_reg_val(input: &str) -> IResult<&str, Instruction> {
    let parse_opcode = alt((tag("set"), tag("sub"), tag("mul"), tag("mod")));
    let (rest, (opcode, r, v)) = tuple((
        parse_opcode,
        preceded(tag(" "), parse_register),
        preceded(tag(" "), parse_value)
    ))(input)?;
    let instruction = match opcode {
        "set" => Instruction::Set(r, v),
        "sub" => Instruction::Sub(r, v),
        "mul" => Instruction::Mul(r, v),
        "mod" => Instruction::Mod(r, v),
        _ => unreachable!()
    };
    Ok((rest, instruction))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_instruction_val_val,
        parse_instruction_reg_val,
    ))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list(tag("\n"), parse_instruction)(input)
}

enum ProgramState {
    Running,
    Terminated
}

struct Program {
    instructions: Vec<Instruction>,
    sp: usize,
    registers: HashMap<Register, Number>,
    state: ProgramState,
    mul_count: u32,
}

impl Program {
    fn new(instructions: Vec<Instruction>, part2: bool) -> Program {
        let mut registers = HashMap::new();
        if part2 {
            registers.insert('a', 1);
        }
        Program {
            instructions,
            sp: 0,
            registers,
            state: ProgramState::Running,
            mul_count: 0,
        }
    }

    fn terminate(&mut self) -> () {
        self.state = ProgramState::Terminated;
    }

    fn tick(&mut self) -> () {
        let mut inc_sp = true;

        match &self.instructions[self.sp] {
            Instruction::Set(r, v) => {
                *self.registers.entry(*r).or_insert(0) = v.resolve(&self.registers);
            }
            Instruction::Sub(r, v) => {
                *self.registers.entry(*r).or_insert(0) -= v.resolve(&self.registers);
            }
            Instruction::Mul(r, v) => {
                *self.registers.entry(*r).or_insert(0) *= v.resolve(&self.registers);
                self.mul_count += 1;
            }
            Instruction::Mod(r, v) => {
                *self.registers.entry(*r).or_insert(0) %= v.resolve(&self.registers);
            }
            Instruction::Jnz(v1, v2) => {
                if v1.resolve(&self.registers) != 0 {
                    let offset = v2.resolve(&self.registers);
                    let new_sp = self.sp as Number + offset;
                    if new_sp < 0 || new_sp as usize >= self.instructions.len() {
                        self.terminate();
                    }
                    self.sp = new_sp as usize;
                    inc_sp = false;
                }
            }
        }
        if inc_sp {
            self.sp += 1;
        }
    }

    fn run(&mut self) {
        while let ProgramState::Running = self.state {
            self.tick();
        }
    }
}

fn get_second_operand_if_register(i: &Instruction) -> Option<Register> {
    let f = |v: &Value| if let Value::Register(r) = v { Some(*r) } else { None };
    match i {
        Instruction::Set(_r, v) => f(v),
        Instruction::Sub(_r, v) => f(v),
        Instruction::Mul(_r, v) => f(v),
        Instruction::Mod(_r, v) => f(v),
        Instruction::Jnz(_v, v) => f(v),
    }
}

fn get_first_operand_if_register(i: &Instruction) -> Option<Register> {
    match i {
        Instruction::Set(r, _v) => Some(*r),
        Instruction::Sub(r, _v) => Some(*r),
        Instruction::Mul(r, _v) => Some(*r),
        Instruction::Mod(r, _v) => Some(*r),
        _ => None,
    }
}

fn patch_program(mut input: Vec<Instruction>) -> std::vec::Vec<Instruction> {
    let utility_register = get_first_operand_if_register(&input[13]).unwrap();
    let tested_register = get_second_operand_if_register(&input[13]).unwrap();
    let looping_register = get_second_operand_if_register(&input[11]).unwrap();

    input[11] = Instruction::Set(utility_register, Value::Register(tested_register));
    input[12] = Instruction::Mod(utility_register, Value::Register(looping_register));
    input[13] = Instruction::Jnz(Value::Register(utility_register), Value::Literal(7));
    input[14] = Instruction::Jnz(Value::Literal(1), Value::Literal(11));
    input[22] = Instruction::Sub(utility_register, Value::Literal(499));

    input[8] = Instruction::Jnz(Value::Literal(1), Value::Literal(24));
    input[9] = Instruction::Set(looping_register, Value::Literal(3));
    input[20] = Instruction::Sub(looping_register, Value::Literal(-2));
    input[24] = Instruction::Jnz(Value::Literal(1), Value::Literal(2));
    input[29] = Instruction::Jnz(Value::Literal(1), Value::Literal(1000));

    input.push(Instruction::Set(utility_register, Value::Register(tested_register)));
    input.push(Instruction::Mod(utility_register, Value::Literal(2)));
    input.push(Instruction::Jnz(Value::Register(utility_register), Value::Literal(-25)));
    input.push(Instruction::Jnz(Value::Literal(1), Value::Literal(-10)));

    input
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_rest, input) = parse_instructions(input).unwrap();

    let mut p = Program::new(input.clone(), false);
    p.run();
    println!("The number of times the mul instruction is called in debug mode: {}", p.mul_count);

    let mut p = Program::new(patch_program(input), true);
    p.run();
    println!("The value of the h register after the program ends: {}", p.registers.get(&'h').unwrap());

    Ok(())
}
