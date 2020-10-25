mod lexer;
mod parser;
mod generator;

//use std::env::args;
use std::process::exit;
//use std::fs;

//const PATH: &str = "~/Code/Zle/Edition3/test";
const IS_DEBUG:bool=true;

fn main() {
    eprintln!("zle compiler activated.");
    let option: Vec<String> = std::env::args().collect();
    if option.len() < 2 {
        eprintln!("lack");
        exit(1);
    }
    let  name=&option[1];
    let code=if let Ok(a)=std::fs::read_to_string(name){
        a
    }else{
        eprintln!("No such a file.");
        exit(2);
    };
    eprintln!("This is nightly build.");
//    let token=lexer::read_into_token(code);
    eprintln!("トークナイズできた");
//    let a=parser::expr(&mut token.iter());
    eprintln!("パースできた");

//    generator::gen_asm(a);
    eprintln!("コード生成したよ");
    let a=String::from("543210ああ");
    assert_eq!(543210,lexer::as_long_as_num(&a).unwrap());
    let b=String::from("あｓだ");
    assert_eq!(None,lexer::as_long_as_num(&b));
    let c=String::from("1あいうえあｄ");
    assert_eq!(Some(1),lexer::as_long_as_num(&c));
    assert_eq!(Some(1234),lexer::as_long_as_num(&"1234".to_string()));
}
