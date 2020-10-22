use super::lexer::*;
use super::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Priority {
    Lev0,
    Lev1,
    Lev2,
}

pub struct PriorityVal {
    level: Priority,
    what: lexer::Token,
}

impl PriorityVal {
    pub fn new(lev: Priority, val: lexer::Token) -> Self {
        PriorityVal {
            level: lev,
            what: val,
        }
    }

    pub fn set_level(&mut self, lev: Priority) {
        self.level = lev;
    }

    pub fn get_level(&self) -> &Priority {
        &self.level
    }
    pub fn set_what(&mut self, wh: lexer::Token) {
        self.what = wh;
    }
    pub fn get_what(&self) -> &lexer::Token {
        &self.what
    }
}

/*pub fn prival_into_sorted() -> Vec<lexer::Token> {
    let sorted = Vec::new();
    sorted
}*/

pub fn token_into_priorty(tokens: Vec<lexer::Token>) -> Vec<PriorityVal> {
    let mut result = Vec::new();
    for run in tokens {
        match run {
            Token::NUM(n) => result.push(PriorityVal::new(Priority::Lev0, lexer::Token::NUM(n))),
            Token::ADD => result.push(PriorityVal::new(Priority::Lev1, lexer::Token::ADD)),
            Token::SUB => result.push(PriorityVal::new(Priority::Lev1, lexer::Token::SUB)),
            Token::MUL => {
                result.push(PriorityVal::new(Priority::Lev2, lexer::Token::MUL));
            }
            Token::DIV => {
                result.push(PriorityVal::new(Priority::Lev2, lexer::Token::DIV));
            }
        }
    }

    result
}

pub struct Node {
    left: Box<Node>,
    right: Box<Node>,
    operator: lexer::Token,
}
pub enum Tree {
    Num(i32),
    Ope(Box<Node>),
    EOF,
}

impl Node {
    pub fn token_into_tree(top: &mut Box<Node>, tokens: &mut std::slice::Iter<Token>) -> Tree {
        match tokens.next() {
            None => Tree::EOF,
            Some(what) => {
                //match what{

                //}
                Tree::EOF
            }
            _ => Tree::EOF,
        }
    }
}

pub mod debug {
    fn declare_debug(name: &str) {
        eprintln!("This is Debug Fn !!");
        eprintln!("From: {}", name);
    }
    use super::super::lexer;
}
