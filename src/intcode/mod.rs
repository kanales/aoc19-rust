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

use Parameter::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Parameter<A> {
    Pos(A),
    Imm(A),
}

impl TryFrom<i32> for Parameter<()> {
    type Error = String;

    fn try_from(item: i32) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Pos(())),
            1 => Ok(Imm(())),
            m => Err(format!("Unknown mode {:?}", m).to_string()),
        }
    }
}

impl<T> Parameter<T> {
    #[allow(dead_code)]
    fn map<B, F: Fn(&T) -> B>(&self, f: F) -> Parameter<B> {
        match self {
            Pos(x) => Pos(f(x)),
            Imm(x) => Imm(f(x)),
        }
    }

    fn try_map<B, E, F>(&self, f: F) -> Result<Parameter<B>, E>
    where
        F: Fn(&T) -> Result<B, E>,
    {
        Ok(match self {
            Pos(x) => Pos(f(x)?),
            Imm(x) => Imm(f(x)?),
        })
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

    #[allow(dead_code)]
    fn map<B, F: Fn(&T) -> B>(&self, f: F) -> Opcode<B> {
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

impl TryFrom<i32> for Opcode<Parameter<()>> {
    type Error = String;

    fn try_from(x: i32) -> Result<Opcode<Parameter<()>>, Self::Error> {
        let p = |i| digit(x / 100, i).try_into();

        let a: Parameter<()> = p(0)?;
        let b: Parameter<()> = p(1)?;
        let c: Parameter<()> = p(2)?;
        let op = match x % 100 {
            1 => Add(a, b, c),
            2 => Mul(a, b, c),
            3 => Inp(c),
            4 => Out(a),
            5 => Jnz(a, b),
            6 => Jz(a, b),
            7 => Lt(a, b, c),
            8 => Equ(a, b, c),
            99 => Hlt,
            o => Err(format!("Unknown operation {:?}", o).to_string())?,
        };
        Ok(op)
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    pc: usize,
    intcode: Vec<i32>,
    status: ProcessStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Paused,
    Outputting(i32),
    Awaiting(Parameter<usize>),
    Exit,
}
use ProcessStatus::*;

impl ProcessStatus {
    pub fn out(&self) -> Option<i32> {
        match self {
            Outputting(x) => Some(*x),
            _ => None,
        }
    }
}

impl Process {
    pub fn new(code: Intcode) -> Self {
        let v: Vec<i32> = code.into();
        Process {
            pc: 0,
            intcode: v,
            status: Paused,
        }
    }

    fn set(&mut self, param: &Parameter<usize>, value: i32) -> Result<(), String> {
        match param {
            Pos(x) => Ok(self.intcode[*x] = value),
            Imm(_) => Err("Can't set to immediate value.".to_owned()),
        }
    }

    fn try_set(&mut self, param: &Parameter<i32>, value: i32) -> Result<(), String> {
        match *param {
            Imm(_) => Err("Can't set to immediate value.".to_owned()),
            Pos(p) => {
                let x: usize = p
                    .try_into()
                    .map_err(|_| format!("Invalid index from {:?}", param).to_owned())?;
                Ok(self.intcode[x] = value)
            }
        }
    }

    fn get(&self, param: &Parameter<i32>) -> Result<i32, String> {
        match *param {
            Imm(x) => Ok(x),
            Pos(p) => {
                let x: usize = p
                    .try_into()
                    .map_err(|_| format!("Invalid index from {:?}", param).to_owned())?;
                Ok(self.intcode[x])
            }
        }
    }

    fn jmp(&mut self, pos: usize) {
        self.pc = pos;
    }

    fn inc(&mut self, steps: usize) {
        self.pc += steps;
    }

    fn inc_setter<'a>(&'a self) -> Box<dyn FnMut(&Parameter<()>) -> Parameter<i32> + 'a> {
        let mut i = 0;
        Box::new(move |m| {
            i += 1;
            match m {
                Pos(_) => Pos(self.intcode[self.pc + i]),
                Imm(_) => Imm(self.intcode[self.pc + i]),
            }
        })
    }

    fn populate(&self, code: Opcode<Parameter<()>>) -> Opcode<Parameter<i32>> {
        code.mut_map(&mut self.inc_setter())
    }

    fn current(&self) -> Result<Opcode<Parameter<i32>>, String> {
        let code: Opcode<Parameter<()>> = self.intcode[self.pc].try_into()?;
        let op = self.populate(code);
        Ok(op)
    }

    fn eval(&mut self) -> Evaluation {
        match self.eval_inner() {
            Ok(ev) => ev,
            Err(s) => EvaluationError(s),
        }
    }

    pub fn resume(&mut self) -> ProcessStatus {
        let ev = match self.status {
            Paused | Outputting(_) => self.eval(),
            // Do nothing if not paused
            _ => return self.status,
        };
        match ev {
            Input(dest) => self.status = Awaiting(dest),
            Output(o) => self.status = Outputting(o),
            EvaluationError(s) => {
                println!("{}", s);
                self.status = Exit
            }
            Halt => self.status = Exit,
        };
        self.status
    }

    pub fn feed(&mut self, input: i32) -> ProcessStatus {
        match self.status {
            Awaiting(dest) => {
                self.set(&dest, input).unwrap();
                self.status = Paused;
                self.status
            }
            _ => panic!(format!("Trying to feed {:?}", self.status)),
        }
    }

    pub fn head(&self) -> i32 {
        self.intcode[0]
    }

    fn eval_inner(&mut self) -> Result<Evaluation, String> {
        let curr = self.current()?;
        let ev = match curr {
            Add(a, b, c) => {
                self.try_set(&c, self.get(&a)? + self.get(&b)?)?;
                self.inc(4);
                self.eval()
            }
            Mul(a, b, c) => {
                let p1 = self.get(&a)?;
                let p2 = self.get(&b)?;
                self.try_set(&c, p1 * p2)?;
                self.inc(4);
                self.eval()
            }
            Out(a) => {
                self.inc(2);
                Output(self.get(&a)?)
            }
            // ugly af
            Inp(a) => {
                self.inc(2);
                Input(a.try_map(|&x| x.try_into()).unwrap())
            }
            Jnz(a, b) => {
                if self.get(&a)? != 0 {
                    self.jmp(self.get(&b)?.try_into().unwrap()); // TODO fix this
                } else {
                    self.inc(3);
                }
                self.eval()
            }
            Jz(a, b) => {
                if self.get(&a)? == 0 {
                    self.jmp(self.get(&b)?.try_into().unwrap()); // TODO fix this
                } else {
                    self.inc(3);
                }
                self.eval()
            }
            Lt(a, b, c) => {
                self.try_set(&c, if self.get(&a) < self.get(&b) { 1 } else { 0 })?;
                self.inc(4);
                self.eval()
            }
            Equ(a, b, c) => {
                self.try_set(&c, if self.get(&a) == self.get(&b) { 1 } else { 0 })?;
                self.inc(4);
                self.eval()
            }
            Hlt => Halt,
        };
        Ok(ev)
    }

    pub fn status(&self) -> ProcessStatus {
        self.status
    }
}

use Evaluation::*;
pub enum Evaluation {
    Input(Parameter<usize>),
    Output(i32),
    Halt,
    EvaluationError(String),
}

impl<'a> fmt::Debug for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input(dest) => write!(f, "Input({:?})", dest),
            Output(o) => write!(f, "Output({})", o),
            Halt => write!(f, "Halt"),
            EvaluationError(err) => write!(f, "EvaluationError {}", err),
        }
    }
}

pub trait Runnable {
    fn run(&mut self, input: i32) -> Option<i32>;
}

// #[derive(Debug, Clone)]
// pub struct ProcessChain(Vec<Process>);

// use std::slice::IterMut;

// impl ProcessChain {
//     pub fn iter_mut(&mut self) -> IterMut<'_, Process> {
//         self.0.iter_mut()
//     }

//     pub fn cycle(&mut self, init: i32) -> Option<i32> {
//         let mut outputs: Vec<Option<i32>> = vec![None; self.0.len()];
//         let mut res: i32 = init;
//         let mut cont = true;
//         while cont {
//             cont = false;
//             for (i, p) in self.0.iter_mut().enumerate() {
//                 match p.feed(res) {
//                     Outputting(x) => {
//                         res = x;
//                         outputs[i] = Some(x);
//                         cont = true
//                     }
//                     _ => {
//                         res = outputs[i]?;
//                     }
//                 }
//             }
//         }
//         Some(res)
//     }
// }

// use std::iter::FromIterator;

// impl<'a> FromIterator<Process> for ProcessChain {
//     fn from_iter<T>(iter: T) -> Self
//     where
//         T: IntoIterator<Item = Process>,
//     {
//         ProcessChain(iter.into_iter().collect())
//     }
// }

// impl IntoIterator for ProcessChain {
//     type Item = Process;
//     type IntoIter = std::vec::IntoIter<Process>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl Runnable for ProcessChain {
//     fn run(&mut self, input: i32) -> Option<i32> {
//         let mut it = self.iter_mut();
//         let res = it.next()?.feed(input).out()?;
//         it.try_fold(res, |r, p| p.feed(r).out())
//     }
// }
