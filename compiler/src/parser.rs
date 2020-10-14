use super::*;
use super::lexer::*;
pub enum Tree{
    
}

pub fn token_into_tree(tokens:Vec<lexer::Token>){

}

//木構造は前失敗したので
//優先度をつけた配列で挑んで見る
pub struct PriorityVal{
    prioity:i32,
    what:lexer::Token,
}

impl PriorityVal{
    pub fn new(pri:i32,val:lexer::Token)->Self{
        PriorityVal{
            prioity:pri,
            what:val,
        }
    }
}

pub fn prival_into_sorted()->Vec<lexer::Token>{
    let sorted=Vec::new();
    
    sorted
}

pub fn token_into_priorty(tokens:Vec<lexer::Token>)->Vec<PriorityVal>{
    let mut result=Vec::new();
    let mut counter=0i32;
    let mut over=0i32;
    for run in tokens{
        match run{
            Token::NUM(n)=>result.push(PriorityVal::new(counter,lexer::Token::NUM(n))),
            Token::ADD=>result.push(PriorityVal::new(counter, lexer::Token::ADD)),
            Token::SUB=>result.push(PriorityVal::new(counter, lexer::Token::SUB)),
            Token::MUL=>{
                result.push(PriorityVal::new(counter-over, lexer::Token::MUL));
                over+=1;
            }
            Token::DIV=>{
                result.push(PriorityVal::new(counter-over, lexer::Token::DIV));
                over+=1;
            }
        }
        counter+=1;
    }

    result
}
