use super::*;

pub enum Token{
    ADD,
    SUB,
    MUL,
    DIV,
    NUM(i32),
}


pub fn read_into_token(input:String)->Vec<Token>{
    let mut result=Vec::new();
    for run in input.split_whitespace(){
        if let Ok(n)=run.parse::<i32>(){
            if IS_DEBUG{
                eprintln!("Num: {}",n);
            }
            result.push(Token::NUM(n));
        }else{
            match run {
                "+"=>result.push(Token::ADD),
                "-"=>result.push(Token::SUB),
                "*"=>result.push(Token::MUL),
                "/"=>result.push(Token::DIV),
                _=>{
                    eprintln!("Unknown Token: {}",run);
                    exit(5);
                }
            }
        }
    }
    result
}