use super::parser::*;
use super::lexer::*;
use super::*;
use std::collections::VecDeque;

//pub fn prival_into_bytecode(mut prival: Vec<PriorityVal>) {}

pub fn gen_asm(token:Box<Node>){
    prelude();
    _what_gen_asm(token);
    println!("  pop rax");
    println!("  ret");
}

fn prelude() {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    return ();
}
fn _what_gen_asm(token:Box<Node>) {
    match  *token.clone(){
        Node::Num(a)=>{
            println!("push {}",a);
            return;
        }
        Node::Ope(b)=>{
            _what_gen_asm(b.left);
            _what_gen_asm(b.right);
        }
    }
    println!("  pop rdi");
    println!("  pop rax");
    if let Node::Ope(a)=*token{
        if let lexer::Token::OPE(op)= a.what{
            match op{
                lexer::Operator::ADD=>{
                    println!("  add rax, rdi");
                }
                Operator::SUB=>{
                    println!("  sub rax, rdi");
                }
                Operator::MUL=>{
                    println!("  imul rax, rdi");
                }
                Operator::DIV=>{
                    println!("  cqo");
                    println!("  idiv rdi");
                }
            }
        }
    }
    println!("push rax");
}
