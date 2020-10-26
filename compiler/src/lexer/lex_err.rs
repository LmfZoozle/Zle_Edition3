use super::*;

fn longline(){
    eprintln!("---------------------------------------");
}

pub fn unknown_token(line:i32,what:char){
    longline();
    eprintln!("An error occurred during lex.");
    eprintln!("Unknown token '{}' found, at {}th token.",what,line);
    longline()
}