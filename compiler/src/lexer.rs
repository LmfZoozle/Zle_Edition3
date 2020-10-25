use super::*;

#[derive(Clone,Copy,PartialEq)]
pub enum Operators{
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
    Ope(Operators),    
    Num(i32),
    Braket(Brackets),
}

pub fn as_long_as_num(input:&String)->Option<i32>{
    //let mut chars=input.chars().collect::<Vec<char>>();
    //流石にこのままの実装は手間かかりすぎダヨナー
    let lim=input.len() as i32+1;
    let mut c=0;
    let mut temporal=0;
    let mut done=false;
    while c<lim{
        let mut buff=String::new();
        let mut inner=0;
        for run in input.chars(){
            buff.push(run);
            inner+=1;
            if inner==c{
                break;
            }
        }
        if let Ok(num) =buff.to_string().parse::<i32>(){
            temporal=num;
            done=true;
        }
        c+=1;
    }

    if done{
        Some(temporal)        
    }else{
        None
    }
}

pub fn Ex_read_into_token(input:String)->Vec<Token>{
    let mut result=Vec::new();
    
    result
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
                    result.push(Token::Ope(Operators::Add))
                }
                "-"=>result.push(Token::Ope(Operators::Sub)),
                "*"=>result.push(Token::Ope(Operators::Mul)),
                "/"=>result.push(Token::Ope(Operators::Div)),
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