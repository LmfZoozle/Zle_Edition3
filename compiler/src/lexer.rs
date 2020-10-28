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

fn consume_numeric(itr: &mut std::slice::Iter<char>) -> Option<i32> {
    let mut numerics = String::new();
    let clo = itr.clone();
    for run in clo {
        if run.is_digit(10) {
            numerics.push(*run);
            itr.next();
        } else {
            break;
        }
    }

    if let Ok(num) = numerics.parse::<i32>() {
        eprint!("{} ",num);
        Some(num)
    } else {
        None
    }
}

fn consume_unknown(itr: &mut std::slice::Iter<char>)->String{
    let mut result=String::new();
    loop{
        if let Some(what)=itr.next(){
            if !what.is_whitespace(){
                result.push(*what);
            }else{
                eprint!("{} ",&result);
                return result;
            }
        }else{
            eprint!("{} ",&result);
            return result;
        }
    }
}

fn consume(itr: &mut std::slice::Iter<char>, expect: &str) -> bool {
    let mut current = String::new();
    let mut clo = itr.clone();
    let mut calltime = 0;
    loop {
        if let Some(what) = clo.next() {
            current.push(*what);
            calltime += 1;
        } else {
            return false;
        }
        if current == expect {
            for _a in 0..calltime {
                eprint!("{} ", itr.next().unwrap());
            }
            return true;
        }
    }
}

pub fn read_into_token(input: String) -> Vec<Token> {
    use Operators::*;
    let mut result = Vec::new();
    let aa: Vec<char> = input.chars().collect();
    let mut iter = aa.iter();
    let mut tokencount = 1;
    let mut blankflag;

    loop {
        blankflag=false;
        if let Some(num) = consume_numeric(&mut iter) {
            result.push(Token::Num(num));
        } else if consume(&mut iter, "+") {
            result.push(Token::Ope(Operators::Add));
        } else if consume(&mut iter, "-") {
            result.push(Token::Ope(Sub));
        } else if consume(&mut iter, "*") {
            result.push(Token::Ope(Mul));
        } else if consume(&mut iter, "/") {
            result.push(Token::Ope(Div))
        } else if consume(&mut iter, "(") {
            result.push(Token::Braket(Brackets::LeftRound));
        } else if consume(&mut iter, ")") {
            result.push(Token::Braket(Brackets::RightRound));
        }else if consume(&mut iter, " "){
            blankflag=true;            
        } 
        else {
            error::from_lex::err_unknown_token(tokencount, &consume_unknown(&mut iter));
        }
        if let None=iter.clone().next(){
            break;
        }
        if !blankflag{
            tokencount+=1;
        }

    }

    eprintln!();
    result

}
