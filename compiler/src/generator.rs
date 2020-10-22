use super::parser::*;
use super::*;
use std::collections::VecDeque;

//pub fn prival_into_bytecode(mut prival: Vec<PriorityVal>) {}

fn prelude() {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    return ();
}

pub fn notyet(mut tokitr: std::slice::Iter<PriorityVal>) -> i32 {
    if let Some(a) = tokitr.next() {
        if let Some(b) = tokitr.next() {
            28
        } else {
            1
        }
    } else {
        0
    }
}

pub fn is_upper(mut tokitr: std::slice::Iter<PriorityVal>) -> bool {
    let ahead = tokitr.next().unwrap();
    let rear = tokitr.next().unwrap();
    if ahead.get_level().priority_into_num() <= rear.get_level().priority_into_num() {
        true
    } else {
        false
    }
}

fn get_number(token: &Vec<PriorityVal>) -> VecDeque<i32> {
    let mut result = VecDeque::new();
    for run in token {
        if let lexer::Token::NUM(a) = *run.get_what() {
            result.push_back(a);
        }
    }
    result
}

fn get_operator(token: &Vec<PriorityVal>)->VecDeque<lexer::Token>{
    use lexer::Token::*;
    let mut result=VecDeque::new();
    for run in token{
        if let lexer::Token::NUM(a)=*run.get_what(){
            //
        }else{
            result.push_back(*run.get_what());
        }
    }
    result
}

/*pub fn get_sorted_operator(token:&Vec<PriorityVal>)->Vec<PriorityVal>{
    let deque=get_operator(token);

}*/

pub fn gen_asm(token: Vec<PriorityVal>) {
    prelude();
    let mut opeiter=get_operator(&token).iter();
    let mut numiter=get_number(&token).iter();
    loop{
        
    }
}
