use super::parser::*;
use super::*;

//pub fn prival_into_bytecode(mut prival: Vec<PriorityVal>) {}

fn prelude() {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    return ()
}

pub fn notyet(mut tokitr:std::slice::Iter<PriorityVal>)->i32{
    if let Some(a)=tokitr.next(){
        if let Some(b)=tokitr.next(){
            2
        }else{
            1
        }
    }else{
        0
    }
}

pub fn is_upper(mut tokitr:std::slice::Iter<PriorityVal>)->bool{
    let ahead=tokitr.next().unwrap();
    let rear=tokitr.next().unwrap();
    if ahead.get_level().priority_into_num()<=rear.get_level().priority_into_num(){
        true
    }else{
        false
    }

}

pub fn gen_asm(token:Vec<PriorityVal>){
    prelude();
    for run in token.iter(){

    }
}