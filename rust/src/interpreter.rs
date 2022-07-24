use crate::parser::{Block, IR};
use std::io::{stdin, stdout, Read, Write};

pub struct Interpreter {
    memory: [u8; 30000],
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: [0; 30000],
            pointer: 0,
        }
    }

    pub fn run(&mut self, block: &Block) {
        for node in &block.0 {
            match node {
                IR::IncrementDP(x) => self.pointer += *x as usize,
                IR::DecrementDP(x) => self.pointer -= *x as usize,
                IR::Increment(x) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*x)
                }
                IR::Decrement(x) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*x)
                }
                IR::Output => {
                    print!("{}", self.memory[self.pointer] as char);
                    // stdio should always work
                    stdout().flush().unwrap();
                }
                IR::Input => {
                    let mut buf = [0; 1];
                    // stdio should always work
                    stdin().read_exact(&mut buf).unwrap();
                    self.memory[self.pointer] = buf[0];
                }
                IR::While(while_block) => {
                    while self.memory[self.pointer] != 0 {
                        self.run(&while_block)
                    }
                }
            }
        }
    }
}
