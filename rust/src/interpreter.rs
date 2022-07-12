use crate::parser::{Block, Node};
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
                Node::IncrementDP(x) => self.pointer += *x as usize,
                Node::DecrementDP(x) => self.pointer -= *x as usize,
                Node::Increment(x) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*x)
                }
                Node::Decrement(x) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*x)
                }
                Node::Output => {
                    print!("{}", self.memory[self.pointer] as char);
                    // stdio should always work
                    stdout().flush().unwrap();
                }
                Node::Input => {
                    let mut buf = [0; 1];
                    // stdio should always work
                    stdin().read_exact(&mut buf).unwrap();
                    self.memory[self.pointer] = buf[0];
                }
                Node::While(while_block) => {
                    while self.memory[self.pointer] != 0 {
                        self.run(&while_block)
                    }
                }
            }
        }
    }
}
