mod lexer;
mod parser;

use std::env::args;
use std::process::exit;

const PATH: &str = "~/Code/Zle/Edition3/test";
const IS_DEBUG:bool=true;

fn main() {
    eprintln!("zle compiler activated.");
    let option: Vec<String> = std::env::args().collect();
    if option.len() < 2 {}

}
