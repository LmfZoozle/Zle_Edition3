use super::*;

#[derive(Clone,Copy,PartialEq)]
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone,Copy,PartialEq)]
pub enum Brackets{
    LeftRound,
    RightRound,
}

#[derive(Clone,Copy,PartialEq)]
pub enum Token{
    Ope(Operator),    
    Num(i32),
    Braket(Brackets),
}


pub fn read_into_token(input:String)->Vec<Token>{
    let mut result=Vec::new();
    for run in input.split_whitespace(){
        if let Ok(n)=run.parse::<i32>(){
            if IS_DEBUG{
                eprintln!("Num: {}",n);
            }
            result.push(Token::Num(n));
        }else{
            match run {
                "+"=>{
                    eprintln!("あど");
                    result.push(Token::Ope(Operator::Add))
                }
                "-"=>result.push(Token::Ope(Operator::Sub)),
                "*"=>result.push(Token::Ope(Operator::Mul)),
                "/"=>result.push(Token::Ope(Operator::Div)),
                "("=>result.push(Token::Braket(Brackets::LeftRound)),
                ")"=>result.push(Token::Braket(Brackets::RightRound)),
                _=>{
                    eprintln!("Unknown Token: {}",run);
                    exit(5);
                }
            }
        }
    }
    result
}