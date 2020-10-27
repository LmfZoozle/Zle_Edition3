use super::*;

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

#[derive(Clone, Copy, PartialEq)]
pub enum Relation {
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
    Return,
    WhiteSpace,
}

fn consume_numeric(itr:&mut std::slice::Iter<char>)->Option<i32>{
    let mut numerics = String::new();
    let clo=itr.clone();
    for run in  clo{
        if run.is_digit(10) {
            numerics.push(*run);
            clo.next();
        } else {
            break;
        }
    }

    if let Ok(num) = numerics.parse::<i32>() {
        Some(num)
    } else {
        None
    }    

}

fn consume<T:Clone>(itr:&mut std::slice::Iter<char>,expect:&str)->bool{
    let mut current=String::new();
    let clo=itr.clone();
    let mut calltime=0;
    loop{
        if let Some(what)=clo.next(){
            current.push(*what);
            calltime+=1;
        }else{
            return false;
        }
        if current==expect{
            for _a in 0..calltime{
                eprint!("{}", itr.next().unwrap());
                eprintln!("であってるか？");
            }
            
            return true;
        }

    }
}

pub fn read_into_token(input: String) -> Vec<Token> {
    let mut result = Vec::new();
    let aa: Vec<char> = input.chars().collect();
    let mut iter = aa.iter();
    let mut tokencount = 0;



}
