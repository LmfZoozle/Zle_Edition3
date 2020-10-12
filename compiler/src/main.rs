mod lexer;
mod parser;

use std::env::args;
use std::process::exit;

const PATH: &str = "~/Code/Zle/Edition3/test";
const IS_DEBUG:bool=true;

//const NAME_ZLE:&str="test.zle";
//const NAME_BYTECODE:&str="test.bc";

fn main() {
    eprintln!("zle compiler activated.");
    let option: Vec<String> = std::env::args().collect();
    if option.len() < 2 {}
    println!("This is from zle compiler");
    println!("")
}
