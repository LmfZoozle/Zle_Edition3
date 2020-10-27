use super::*;
//mod lex_err;

#[derive(Clone, Copy, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Brackets {
    LeftRound,
    RightRound,
}

#[derive(Clone,Copy,PartialEq)]
pub enum Relation{
    Equal,
    Not,
    NotEq,
    Assign,
    Bigger,
    BigOrEq,
    Smaller,
    SmallOrEq,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    Ope(Operators),
    Num(i32),
    Braket(Brackets),
    Ident(char),
    Relate(Relation),
}

//軽量化した
fn as_long_as_num(input: &String) -> Option<i32> {
    let mut numerics=String::new();
    for run in input.chars(){
        if run.is_digit(10){
            numerics.push(run);
        }else{
            break;
        }
    }
    if let Ok(num)=numerics.parse::<i32>(){
        Some(num)
    }else{
        None
    }
}



pub fn read_into_token(input: String) -> Vec<Token> {
    let mut result = Vec::new();
    let aa: Vec<char> = input.chars().collect();
    let mut iter = aa.iter();
    let mut numerics = String::new();
    let mut havenum = false;
    let mut tokencount=0;
    loop {
        if let Some(run) = iter.next() {
            match run {
                _ if run.is_numeric() => {
                    eprintln!("すうじ！");
                    numerics.push(*run);
                    havenum = true;
                }

                _ if 'a'<=*run&&*run<='z'=>{
                    eprintln!("いちもじ！");
                    if havenum {
                        havenum = false;
                        result.push(Token::Ident(*run));
                        numerics.clear();
                        tokencount+=1;
                    }
                    result.push(Token::Ident(*run));
                    tokencount+=1;
                }

                '+' => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();
                        tokencount+=1;
                    }
                    result.push(Token::Ope(Operators::Add));
                    tokencount+=1;
                }
                '-' => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();
                        tokencount+=1;
                    }
                    result.push(Token::Ope(Operators::Sub));
                        tokencount+=1;
                }
                '*' => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();
                        tokencount+=1;
                    }
                    result.push(Token::Ope(Operators::Mul));
                        tokencount+=1;
                }
                '/' => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();
                        tokencount+=1;
                    }
                    result.push(Token::Ope(Operators::Div));
                        tokencount+=1;
                }
                '(' => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();
                        tokencount+=1;
                    } 
                    result.push(Token::Braket(Brackets::LeftRound));
                        tokencount+=1;
                }
                ')' => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();                   
                        tokencount+=1;
                    } 
                        result.push(Token::Braket(Brackets::RightRound));                    
                        tokencount+=1;
                }
                ' '=>{
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();                   
                        tokencount+=1;
                    }
                }
                _ => {
                    if havenum {
                        havenum = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();                   
                        tokencount+=1;
                    } 
                    tokencount+=1;
                    error::from_lex::err_unknown_token(tokencount, *run);                    
                }
            }
        } else {
            if havenum{
                result.push(Token::Num(numerics.parse::<i32>().unwrap()));
            }
            break;
        }

    }
    result
}

