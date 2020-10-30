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
    use lexer::Token::*;
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
        match a.what{
            Add=>{
                println!("  add rax, rdi");
            }
            Sub=>{
                println!("  sub rax, rdi");
            }
            Mul=>{
                println!("  imul rax, rdi");
            }
            Div=>{
                println!("  cqo");
                println!("  imul rax, rdi");
            }
            Equal=>{
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al");
            }
            NotEq=>{
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al");
            }
            Smaller=>{
                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzb rax, al");
            }
            SmallOrEq=>{
                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al");
            }
            _=>{
                eprintln!("Something went wrong");
                std::process::exit(4);
            }            
        }
    }
    println!("push rax");
}
