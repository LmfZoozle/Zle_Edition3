use super::*;

#[derive(Clone,Copy)]
pub enum Operator{
    ADD,
    SUB,
    MUL,
    DIV,
}
#[derive(Clone,Copy)]
pub enum Token{
    OPE(Operator),    
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
                "+"=>result.push(Token::OPE(Operator::ADD)),
                "-"=>result.push(Token::OPE(Operator::SUB)),
                "*"=>result.push(Token::OPE(Operator::MUL)),
                "/"=>result.push(Token::OPE(Operator::DIV)),
                _=>{
                    eprintln!("Unknown Token: {}",run);
                    exit(5);
                }
            }
        }
    }
    result
}