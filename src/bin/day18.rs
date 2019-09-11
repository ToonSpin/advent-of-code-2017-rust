use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, VecDeque};

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
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
    Jgz(Value, Value),
}

fn parse_literal(input: &str) -> IResult<&str, Number> {
    let r = recognize(pair(opt(tag("-")), digit1));
    map_res(r, str::parse::<Number>)(input)
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    one_of("abcdefghijklmnopqrstuvwxyz")(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let parse_register_as_value = map(parse_register, |r| Value::Register(r));
    let parse_literal_as_value = map(parse_literal, |n| Value::Literal(n));
    alt((parse_register_as_value, parse_literal_as_value))(input)
}

fn parse_instruction_val_val(input: &str) -> IResult<&str, Instruction> {
    let parse_instr = preceded(tag("jgz "), separated_pair(parse_value, tag(" "), parse_value));
    let (rest, (v1, v2)) = parse_instr(input)?;
    Ok((rest, Instruction::Jgz(v1, v2)))
}

fn parse_instruction_reg_val(input: &str) -> IResult<&str, Instruction> {
    let parse_opcode = alt((tag("set"), tag("add"), tag("mul"), tag("mod")));
    let (rest, (opcode, r, v)) = tuple((
        parse_opcode,
        preceded(tag(" "), parse_register),
        preceded(tag(" "), parse_value)
    ))(input)?;
    let instruction = match opcode {
        "set" => Instruction::Set(r, v),
        "add" => Instruction::Add(r, v),
        "mul" => Instruction::Mul(r, v),
        "mod" => Instruction::Mod(r, v),
        _ => unreachable!()
    };
    Ok((rest, instruction))
}

fn parse_instruction_send(input: &str) -> IResult<&str, Instruction> {
    let parse_send = preceded(tag("snd "), parse_value);
    map(parse_send, |v| Instruction::Snd(v))(input)
}

fn parse_instruction_receive(input: &str) -> IResult<&str, Instruction> {
    let parse_receive = preceded(tag("rcv "), parse_register);
    map(parse_receive, |r| Instruction::Rcv(r))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_instruction_val_val,
        parse_instruction_reg_val,
        parse_instruction_send,
        parse_instruction_receive,
    ))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list(tag("\n"), parse_instruction)(input)
}

enum ProgramState {
    Running,
    WaitingForMessage,
    Terminated
}

struct Program {
    instructions: Vec<Instruction>,
    sp: usize,
    registers: HashMap<Register, Number>,
    state: ProgramState,
    message_queue: VecDeque<Number>,
    last_message_sent: Option<Number>,
    do_part1: bool,
}

impl Program {
    fn new(program_id: Number, instructions: Vec<Instruction>) -> Program {
        let mut registers = HashMap::new();
        registers.insert('p', program_id);
        Program {
            instructions,
            sp: 0,
            registers,
            state: ProgramState::Running,
            message_queue: VecDeque::new(),
            last_message_sent: None,
            do_part1: program_id == 0,
        }
    }

    fn terminate(&mut self) -> () {
        self.state = ProgramState::Terminated;
    }

    fn send_message(&mut self, message: Number) -> () {
        self.message_queue.push_back(message);
    }

    fn tick(&mut self) -> Option<Number> {
        let mut inc_sp = true;
        let mut message = None;

        match &self.instructions[self.sp] {
            Instruction::Set(r, v) => {
                *self.registers.entry(*r).or_insert(0) = v.resolve(&self.registers);
            }
            Instruction::Add(r, v) => {
                *self.registers.entry(*r).or_insert(0) += v.resolve(&self.registers);
            }
            Instruction::Mul(r, v) => {
                *self.registers.entry(*r).or_insert(0) *= v.resolve(&self.registers);
            }
            Instruction::Mod(r, v) => {
                *self.registers.entry(*r).or_insert(0) %= v.resolve(&self.registers);
            }
            Instruction::Snd(v) => {
                message = Some(v.resolve(&self.registers));
                self.last_message_sent = message;
            }
            Instruction::Rcv(r) => {
                let register_value: &mut i64 = self.registers.entry(*r).or_insert(0);

                if self.do_part1 && *register_value > 0 {
                    if let Some(m) = self.last_message_sent {
                        println!("The first frequency recovered: {}", m);
                        self.do_part1 = false;
                    }
                }

                match self.message_queue.pop_front() {
                    Some(message) => {
                        *register_value = message;
                    }
                    None => {
                        self.state = ProgramState::WaitingForMessage;
                        inc_sp = false;
                    }
                }
            }
            Instruction::Jgz(v1, v2) => {
                if v1.resolve(&self.registers) > 0 {
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
        message
    }
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];


    let (_rest, input) = parse_instructions(input).unwrap();

    let mut p0 = Program::new(0, input.clone());
    let mut p1 = Program::new(1, input.clone());
    let mut p1_send_counter = 0;

    loop {
        if let ProgramState::WaitingForMessage =  p0.state {
            if let ProgramState::WaitingForMessage =  p1.state {
                p0.terminate();
                p1.terminate();
            }
        }

        if let ProgramState::Terminated = p1.state {
            break;
        }

        let p0_message = p0.tick();
        let p1_message = p1.tick();

        if let Some(m) = p0_message {
            p1.send_message(m);
        }

        if let Some(m) = p1_message {
            p1_send_counter += 1;
            p0.send_message(m);
        }
    }

    println!("Number of messages sent by program 1: {}", p1_send_counter);

    Ok(())
}
