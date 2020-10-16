mod lexer;
mod parser;
mod generator;

//use std::env::args;
use std::process::exit;
//use std::fs;

const PATH: &str = "~/Code/Zle/Edition3/test";
const IS_DEBUG:bool=true;

fn main() {
    eprintln!("zle compiler activated.");
    let option: Vec<String> = std::env::args().collect();
    if option.len() < 2 {
        eprintln!("lack");
        exit(1);
    }
    println!("This is from zle compiler");
    let  name=&option[1];
    let code=if let Ok(a)=std::fs::read_to_string(name){
        a
    }else{
        eprintln!("No such a file.");
        exit(2);
    };
    eprintln!("This is nightly build.");
    let token=lexer::read_into_token(code);
    let priority=parser::token_into_priorty(token);
    let a=vec!(4,4,0,2,0,6,5,0,1);
    for run in a.split(|x|*x==0){
        eprintln!("{:?}だ",run);
    }
    let a=vec!(22,4,22,4,4,4,4,33,4,8,88);
    eprintln!("こっから");
    for run in a.split(|x|*x==4){
        eprintln!("{:?}だ",run);
    }
    
}
