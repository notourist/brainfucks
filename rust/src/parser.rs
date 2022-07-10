use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Block, &'static str> {
        let mut block = Block::new();
        let mut i = self.index;
        while i < self.tokens.len() {
            self.index += 1;
            match &self.tokens[i] {
                Token::GreaterThan => block.push(Node::IncrementDP),
                Token::SmallerThan => block.push(Node::DecrementDP),
                Token::Plus => block.push(Node::Increment),
                Token::Minus => block.push(Node::Decrement),
                Token::Point => block.push(Node::Output),
                Token::Comma => block.push(Node::Input),
                Token::BracketOpen => {
                    let while_block = self.parse()?;
                    block.push(Node::While(while_block));
                    // hacky time, "fix" i after index was incremented in a recursive function call
                    // i needs to be decreased by one, as the recursive call will set self.index
                    // one character to far.
                    i = self.index - 1
                }
                Token::BracketClose => break,
            }
            i += 1;
        }
        Ok(block)
    }
}

pub struct Block(pub Vec<Node>);

impl Block {
    fn new() -> Block {
        Block(Vec::new())
    }

    fn push(&mut self, node: Node) {
        self.0.push(node);
    }

    pub fn to_string(&self, prev: &String) -> String {
        let mut string = String::from(prev);
        for node in &self.0 {
            match node {
                Node::IncrementDP => string += ">",
                Node::DecrementDP => string += "<",
                Node::Increment => string += "+",
                Node::Decrement => string += "-",
                Node::Output => string += ".",
                Node::Input => string += ",",
                Node::While(while_block) => {
                    string += "[";
                    string += &while_block.to_string(&String::new());
                    string += "]";
                }
            }
        }
        string
    }
}

pub enum Node {
    IncrementDP,
    DecrementDP,
    Increment,
    Decrement,
    Output,
    Input,
    While(Block),
}
