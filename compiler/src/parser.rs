use super::lexer::*;
use super::*;

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
    pub fn token_into_tree(top:&mut Box<Node>,tokens: &mut std::slice::Iter<Token>)->Tree {
        match tokens.next(){
            None=>{
                Tree::EOF
            }
            Some(what)=>{
                match what{
                    
                }
            }
            _=>{
                Tree::EOF
            }
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
