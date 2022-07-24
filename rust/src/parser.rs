use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    current_node: Option<IR>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0, current_node: None }
    }

    pub fn parse(&mut self) -> Result<Block, &'static str> {
        let mut block = Block::new();
        let mut i = self.index;
        while i < self.tokens.len() {
            self.index += 1;
            match self.tokens[i] {
                Token::GreaterThan => {
                    match &self.current_node {
                        None => self.current_node = Some(IR::IncrementDP(1)),
                        Some(prev) => {
                            if let IR::IncrementDP(x) = prev {
                                self.current_node = Some(IR::IncrementDP(x + 1))
                            } else {
                                block.push(prev.clone());
                                self.current_node = Some(IR::IncrementDP(1))
                            }
                        }
                    }
                }
                Token::SmallerThan => {
                    match &self.current_node {
                        None => self.current_node = Some(IR::DecrementDP(1)),
                        Some(prev) => {
                            if let IR::DecrementDP(x) = prev {
                                self.current_node = Some(IR::DecrementDP(x + 1))
                            } else {
                                block.push(prev.clone());
                                self.current_node = Some(IR::DecrementDP(1))
                            }
                        }
                    }
                }
                Token::Plus =>
                    match &self.current_node {
                        None => self.current_node = Some(IR::Increment(1)),
                        Some(prev) => {
                            if let IR::Increment(x) = prev {
                                self.current_node = Some(IR::Increment(x + 1))
                            } else {
                                block.push(prev.clone());
                                self.current_node = Some(IR::Increment(1))
                            }
                        }
                    }
                Token::Minus => {
                    match &self.current_node {
                        None => self.current_node = Some(IR::Decrement(1)),
                        Some(prev) => {
                            if let IR::Decrement(x) = prev {
                                self.current_node = Some(IR::Decrement(x + 1))
                            } else {
                                block.push(prev.clone());
                                self.current_node = Some(IR::Decrement(1))
                            }
                        }
                    }
                }
                Token::Point => {
                    self.push_current_node(&mut block);
                    block.push(IR::Output)
                }
                Token::Comma => {
                    self.push_current_node(&mut block);
                    block.push(IR::Input);
                }
                Token::BracketOpen => {
                    self.push_current_node(&mut block);
                    let while_block = self.parse()?;
                    //self.current_node = None;
                    block.push(IR::While(while_block));
                    // hacky time, "fix" i after index was incremented in a recursive function call
                    // i needs to be decreased by one, as the recursive call will set self.index
                    // one character to far.
                    i = self.index - 1
                }
                Token::BracketClose => {
                    self.push_current_node(&mut block);
                    break;
                },
            }
            i += 1;
        }
        Ok(block)
    }

    fn push_current_node(&mut self, block: &mut Block) {
        if let Some(n) = &self.current_node {
            block.push(n.clone());
            self.current_node = None;
        }
    }
}

#[derive(Clone)]
pub struct Block(pub Vec<IR>);

impl Block {
    fn new() -> Block {
        Block(Vec::new())
    }

    fn push(&mut self, node: IR) {
        self.0.push(node);
    }

    pub fn to_string(&self, prev: &String) -> String {
        let mut string = String::from(prev);
        for node in &self.0 {
            match node {
                IR::IncrementDP(x) => string += &">".repeat(*x as usize),
                IR::DecrementDP(x) => string += &"<".repeat(*x as usize),
                IR::Increment(x) => string += &"+".repeat(*x as usize),
                IR::Decrement(x) => string += &"-".repeat(*x as usize),
                IR::Output => string += ".",
                IR::Input => string += ",",
                /*IR::While(while_block) => {
                    string += "[";
                    string += &while_block.to_string(&String::new());
                    string += "]";
                },*/
            }
        }
        string
    }
}

#[derive(Clone)]
pub enum IR {
    IncrementDP(u8),
    DecrementDP(u8),
    Increment(u8),
    Decrement(u8),
    Output,
    Input,
    JumpZero(usize),
    JumpNonZero(usize),
}
