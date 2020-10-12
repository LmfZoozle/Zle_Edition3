mod lexer;
mod parser;

use std::env::args;
use std::process::exit;
use std::fs;

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
    let name=&option[0];
    let code=if let Ok(a)=std::fs::read_to_string(name){
        a
    }else{
        exit(2);
    };
}