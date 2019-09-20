use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res, value},
    multi::separated_list,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

type State = char;

#[derive(Clone, Copy)]
enum Value {
    Zero,
    One,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct Instruction {
    value_to_write: Value,
    direction_to_move: Direction,
    next_state: State,
}

impl Instruction {
    fn new(input: (Value, Direction, State)) -> Instruction {
        let (value_to_write, direction_to_move, next_state) = input;
        Instruction {
            value_to_write,
            direction_to_move,
            next_state,
        }
    }
}

#[derive(Clone)]
struct Rule {
    instr_zero: Instruction,
    instr_one: Instruction,
}

struct TuringMachine {
    rules: HashMap<State, Rule>,
    tape: VecDeque<Value>,
    pos: usize,
    state: State,
}

impl TuringMachine {
    fn new(rules: HashMap<State, Rule>, state: State) -> TuringMachine {
        let mut tape = VecDeque::new();
        tape.push_back(Value::Zero);
        let pos = 0;
        let state = state;
        TuringMachine {
            rules,
            tape,
            pos,
            state,
        }
    }

    fn move_left(&mut self) {
        if self.pos == 0 {
            self.tape.push_front(Value::Zero);
        } else {
            self.pos -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.pos == self.tape.len() - 1 {
            self.tape.push_back(Value::Zero);
        }
        self.pos += 1;
    }

    fn iterate(&mut self) {
        let rule = self.rules.get(&self.state).unwrap();
        let instruction = match self.tape[self.pos] {
            Value::Zero => &rule.instr_zero,
            Value::One => &rule.instr_one,
        };
        self.tape[self.pos] = instruction.value_to_write;
        self.state = instruction.next_state;
        match instruction.direction_to_move {
            Direction::Left => {
                self.move_left();
            }
            Direction::Right => {
                self.move_right();
            }
        }
    }

    fn diagnostic_checksum(&self) -> usize {
        self.tape
            .iter()
            .filter(|&v| if let Value::One = *v { true } else { false })
            .count()
    }
}

fn parse_val_inst(input: &str) -> IResult<&str, Value> {
    delimited(tag("    - Write the value "), parse_value, tag(".\n"))(input)
}

fn parse_dir_inst(input: &str) -> IResult<&str, Direction> {
    delimited(
        tag("    - Move one slot to the "),
        parse_direction,
        tag(".\n"),
    )(input)
}

fn parse_state_inst(input: &str) -> IResult<&str, State> {
    delimited(tag("    - Continue with state "), parse_state, tag(".\n"))(input)
}

fn parse_state_specifier(input: &str) -> IResult<&str, State> {
    delimited(tag("In state "), parse_state, tag(":\n"))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let parse_tuple = tuple((parse_val_inst, parse_dir_inst, parse_state_inst));
    map(parse_tuple, |t| Instruction::new(t))(input)
}

fn parse_zero_value_instruction(input: &str) -> IResult<&str, Instruction> {
    preceded(tag("  If the current value is 0:\n"), parse_instruction)(input)
}

fn parse_one_value_instruction(input: &str) -> IResult<&str, Instruction> {
    preceded(tag("  If the current value is 1:\n"), parse_instruction)(input)
}

fn parse_rule(input: &str) -> IResult<&str, (State, Rule)> {
    let rule_parser = pair(parse_zero_value_instruction, parse_one_value_instruction);
    let rule_parser = map(rule_parser, |(i0, i1)| Rule {
        instr_zero: i0,
        instr_one: i1,
    });
    pair(parse_state_specifier, rule_parser)(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(State, Rule)>> {
    separated_list(char('\n'), parse_rule)(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((value(Value::One, char('1')), value(Value::Zero, char('0'))))(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("left")),
        value(Direction::Right, tag("right")),
    ))(input)
}

fn parse_state(input: &str) -> IResult<&str, State> {
    one_of("ABCDEF")(input)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_prelude(input: &str) -> IResult<&str, (State, u64)> {
    let parser = separated_pair(
        parse_state,
        tag(".\nPerform a diagnostic checksum after "),
        parse_u64,
    );
    delimited(tag("Begin in state "), parser, tag(" steps.\n\n"))(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (input, (state, num_steps)) = parse_prelude(input).unwrap();

    let (_rest, rules) = parse_rules(input).unwrap();
    let mut input = HashMap::new();
    for (state, rule) in rules.iter() {
        input.insert(*state, rule.clone());
    }

    let mut machine = TuringMachine::new(input, state);

    for _i in 0..num_steps {
        machine.iterate();
    }

    println!(
        "The diagnostic checksum after {} steps: {}",
        num_steps,
        machine.diagnostic_checksum()
    );

    Ok(())
}
