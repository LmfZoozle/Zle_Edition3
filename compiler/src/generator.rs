use super::parser::*;
use super::lexer::*;
use super::*;
use std::collections::VecDeque;

//pub fn prival_into_bytecode(mut prival: Vec<PriorityVal>) {}

pub fn gen_asm(token:Box<Node>){
    prologue();
    _what_gen_asm(token);
    epilogue();
}

fn epilogue(){
    println!("  pop rax");
    println!("  ret");
}

fn prologue() {
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
        if let lexer::Token::Ope(op)= a.what{
            match op{
                lexer::Operator::Add=>{
                    println!("  add rax, rdi");
                }
                Operator::Sub=>{
                    println!("  sub rax, rdi");
                }
                Operator::Mul=>{
                    println!("  imul rax, rdi");
                }
                Operator::Div=>{
                    println!("  cqo");
                    println!("  idiv rdi");
                }
            }
        }
    }
    println!("push rax");
}
