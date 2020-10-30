use super::*;


#[derive(Clone, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    LeftRound,    
    RightRound,
    Number(i32),
    Ident(String),
    Assign,
    Equal,
    Not,
    NotEq,
    Smaller,
    SmallOrEq,
    Bigger,
    BigOrEq,
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
        eprint!("{} ", num);
        Some(num)
    } else {
        None
    }
}

fn consume_unknown(itr: &mut std::slice::Iter<char>) -> String {
    let mut result = String::new();
    loop {
        if let Some(what) = itr.next() {
            if !what.is_whitespace() {
                result.push(*what);
            } else {
                eprint!("{} ", &result);
                return result;
            }
        } else {
            eprint!("{} ", &result);
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
            if !what.is_whitespace() {
                current.push(*what);
            }
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

fn consume_ident(itr: &mut std::slice::Iter<char>)->Option<String>{
    let mut current=String::new();
    let clo=itr.clone();    
    for r in clo{
        if r.is_alphanumeric()&&!r.is_whitespace(){
            current.push(*r);
            itr.next();
        }else{
            break;
        }
    }
    if current.is_empty(){
        None
    }else{
        Some(current)
    }
}

pub fn read_into_token(input: String) -> Vec<Token> {

    let mut result = Vec::new();
    let aa: Vec<char> = input.chars().collect();
    let mut iter = &mut aa.iter();
    let mut tokencount = 1;
    let mut blankflag;
    //let refer = &mut iter;

    loop {
        blankflag = false;
        //数字を処理
        if let Some(num) = consume_numeric(&mut iter) {
            result.push(Token::Number(num));
        }
        //四則演算を処理
        else if consume(iter, "+") {
            result.push(Token::Add);
        } else if consume(iter, "-") {
            result.push(Token::Sub);
        } else if consume(iter, "*") {
            result.push(Token::Mul);
        } else if consume(iter, "/") {
            result.push(Token::Div)
        }
        //カッコの類を処理
        else if consume( iter, "(") {
            result.push(Token::LeftRound);
        } else if consume( iter, ")") {
            result.push(Token::RightRound);
        }
        //条件式の類を処理
        else if consume( iter, "<=") {
            result.push(Token::SmallOrEq);
        } else if consume( iter, ">=") {
            result.push(Token::BigOrEq);
        } else if consume( iter, "!=") {
            result.push(Token::NotEq);
        } else if consume( iter, "==") {
            result.push(Token::Equal);
        } else if consume( iter, ">") {
            result.push(Token::Bigger);
        } else if consume( iter, "<") {
            result.push(Token::Smaller);
        } else if consume( iter, "!") {
            result.push(Token::Not);
        } else if consume( iter, "=") {
            result.push(Token::Assign);
        }
        //識別子を処理
        else if let Some(name)=consume_ident(iter){
            result.push(Token::Ident(name));
        }
        //空白を処理 トークンに数えない
        else if consume( iter, " ") {
            blankflag = true;
        }
        //未知のトークン
        else {
            error::from_lex::err_unknown_token(tokencount, &consume_unknown( iter));
        }
        if let None = iter.clone().next() {
            break;
        }
        if !blankflag {
            tokencount += 1;
        }
    }

    eprintln!();
    result
}
