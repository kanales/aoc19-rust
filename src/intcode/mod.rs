pub mod intcode;

pub use intcode::Intcode;

use std::convert::{TryFrom, TryInto};
use std::fmt;

// OPCODE
use Opcode::*;
#[derive(Debug, PartialEq)]
pub enum Opcode<A> {
    Add(A, A, A),
    Mul(A, A, A),
    Inp(A),
    Out(A),
    Jnz(A, A),
    Jz(A, A),
    Lt(A, A, A),
    Equ(A, A, A),
    Hlt,
}

use Mode::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Pos,
    Imm,
}

impl TryFrom<i32> for Mode {
    type Error = String;

    fn try_from(item: i32) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Pos),
            1 => Ok(Imm),
            m => Err(format!("Unknown mode {:?}", m).to_string()),
        }
    }
}

fn digit(x: i32, i: u32) -> i32 {
    x / 10_i32.pow(i) % 10
}

impl<T> Opcode<T> {
    fn mut_map<B>(&self, f: &mut impl FnMut(&T) -> B) -> Opcode<B> {
        match self {
            Add(a, b, c) => Add(f(a), f(b), f(c)),
            Mul(a, b, c) => Mul(f(a), f(b), f(c)),
            Out(a) => Out(f(a)),
            Inp(a) => Inp(f(a)),
            Jnz(a, b) => Jnz(f(a), f(b)),
            Jz(a, b) => Jz(f(a), f(b)),
            Lt(a, b, c) => Lt(f(a), f(b), f(c)),
            Equ(a, b, c) => Equ(f(a), f(b), f(c)),
            _ => Hlt,
        }
    }
}

impl TryFrom<i32> for Opcode<Mode> {
    type Error = String;

    fn try_from(x: i32) -> Result<Opcode<Mode>, Self::Error> {
        let p = |i| digit(x / 100, i).try_into();

        let a = p(0)?;
        let b = p(1)?;
        let op = match x % 100 {
            1 => Add(a, b, Imm),
            2 => Mul(a, b, Imm),
            3 => Inp(Imm),
            4 => Out(a),
            5 => Jnz(a, b),
            6 => Jz(a, b),
            7 => Lt(a, b, Imm),
            8 => Equ(a, b, Imm),
            99 => Hlt,
            o => Err(format!("Unknown operation {:?}", o).to_string())?,
        };
        Ok(op)
    }
}

#[derive(Debug)]
pub struct Process {
    pc: usize,
    intcode: Vec<i32>,
}

impl Process {
    pub fn new(code: Intcode) -> Self {
        let v: Vec<i32> = code.into();
        Process { pc: 0, intcode: v }
    }

    pub fn set(&mut self, index: &usize, value: i32) {
        self.intcode[*index] = value;
    }

    pub fn get(&self, index: &usize) -> i32 {
        self.intcode[*index]
    }

    pub fn jmp(&mut self, pos: usize) {
        self.pc = pos;
    }

    fn inc(&mut self, steps: usize) {
        self.pc += steps;
    }

    fn try_set(&mut self, index: i32, val: i32) -> Result<(), String> {
        let res: Result<usize, _> = index.try_into();
        match res {
            Ok(idx) => Ok(self.intcode[idx as usize] = val),
            Err(_) => Err(format!("Invalid index {:?}", index)),
        }
    }

    fn inc_setter<'a>(&'a self) -> Box<dyn FnMut(&Mode) -> i32 + 'a> {
        let mut i = 0;
        Box::new(move |m| {
            i += 1;
            match m {
                Pos => {
                    let idx: usize = self.intcode[&self.pc + i].try_into().unwrap();
                    self.intcode[idx]
                }
                Imm => self.intcode[self.pc + i],
            }
        })
    }

    fn populate(&self, code: Opcode<Mode>) -> Opcode<i32> {
        code.mut_map(&mut self.inc_setter())
    }

    fn current(&self) -> Result<Opcode<i32>, String> {
        let code: Opcode<Mode> = self.intcode[self.pc].try_into()?;
        let op = self.populate(code);
        Ok(op)
    }

    pub fn run(&mut self) -> Evaluation {
        //println!("{:?}", self);
        match self.run_inner() {
            Ok(ev) => ev,
            Err(s) => EvaluationError(s),
        }
    }
    pub fn run_inner(&mut self) -> Result<Evaluation, String> {
        let curr = self.current();
        //println!("[{}]\t{:?}", self.pc, curr);
        let ev = match curr? {
            Add(a, b, c) => {
                self.try_set(c, a + b)?;
                self.inc(4);
                self.run()
            }
            Mul(a, b, c) => {
                self.try_set(c, a * b)?;
                self.inc(4);
                self.run()
            }
            Out(a) => {
                self.inc(2);
                Output(a, Box::new(self.run()))
            }
            // ugly af
            Inp(a) => match a.try_into() {
                Ok(b) => Input(Box::new(move |i| {
                    self.inc(2);
                    self.set(&b, i);
                    self.run()
                })),
                Err(_) => Err(format!("Invalid index {:?}", a))?,
            },
            Jnz(a, b) => {
                if a != 0 {
                    self.jmp(b.try_into().unwrap());
                } else {
                    self.inc(3);
                }
                self.run()
            }
            Jz(a, b) => {
                if a == 0 {
                    self.jmp(b.try_into().unwrap());
                } else {
                    self.inc(3);
                }
                self.run()
            }
            Lt(a, b, c) => {
                self.try_set(c, if a < b { 1 } else { 0 })?;
                self.inc(4);
                self.run()
            }
            Equ(a, b, c) => {
                self.try_set(c, if a == b { 1 } else { 0 })?;
                self.inc(4);
                self.run()
            }
            Hlt => Halt,
        };
        Ok(ev)
    }
}

use Evaluation::*;
pub enum Evaluation<'a> {
    Input(Box<dyn FnOnce(i32) -> Evaluation<'a> + 'a>),
    Output(i32, Box<Evaluation<'a>>),
    Halt,
    EvaluationError(String),
}

impl<'a> fmt::Debug for Evaluation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input(_) => write!(f, "Input(FnOnce(i32) -> Evaluation)"),
            Output(o, _) => write!(f, "Output({}, Box<Evaluation>)", o),
            Halt => write!(f, "Halt"),
            EvaluationError(err) => write!(f, "EvaluationError {}", err),
        }
    }
}
use std::iter::Iterator;
pub fn run_until_end(
    process: &mut Process,
    it: &mut impl Iterator<Item = i32>,
) -> Result<i32, String> {
    let mut evaluation = process.run();
    let mut output: Result<i32, String> = Err("No output found".to_owned());
    loop {
        match evaluation {
            Evaluation::Input(f) => evaluation = f(it.next().unwrap()),
            Evaluation::Output(o, ev) => {
                output = Ok(o);
                evaluation = *ev;
            }
            Evaluation::Halt => break,
            Evaluation::EvaluationError(e) => return Err(e),
        }
    }
    output
}
