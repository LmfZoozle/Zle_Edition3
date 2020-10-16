use super::*;
use super::lexer::*;
/*pub enum Tree{
    
}

pub fn token_into_tree(tokens:Vec<lexer::Token>){

}
*/


//木構造は前失敗したので
//優先度をつけた配列で挑んで見る
//とりあえずLevNのenumでやる
//->u32でやるよりenumのが整理しやすいかと思ったけど、()とかで優先度が変わるなら
//enumじゃ厳しいかな
#[derive(PartialEq)]
pub enum Priority{
    Lev0,
    Lev1,
    Lev2,
}

pub struct PriorityVal{
    level:Priority,
    what:lexer::Token,
}

impl PriorityVal{
    pub fn new(lev:Priority,val:lexer::Token)->Self{
        PriorityVal{
            level:lev,
            what:val,
        }
    }

    pub fn set_level(&mut self,lev:Priority){
        self.level=lev;
    }

    pub fn get_level(&self)->&Priority{
        &self.level
    }
    pub fn set_what(&mut self,wh:lexer::Token){
        self.what=wh;
    }
    pub fn get_what(&self)->&lexer::Token{
        &self.what
    }
}

pub fn prival_into_sorted()->Vec<lexer::Token>{
    let sorted=Vec::new();
    
    sorted
}

pub fn token_into_priorty(tokens:Vec<lexer::Token>)->Vec<PriorityVal>{
    let mut result=Vec::new();
    for run in tokens{
        match run{
            Token::NUM(n)=>result.push(PriorityVal::new(Priority::Lev0,lexer::Token::NUM(n))),
            Token::ADD=>result.push(PriorityVal::new(Priority::Lev1, lexer::Token::ADD)),
            Token::SUB=>result.push(PriorityVal::new(Priority::Lev1, lexer::Token::SUB)),
            Token::MUL=>{
                result.push(PriorityVal::new(Priority::Lev2, lexer::Token::MUL));
            }
            Token::DIV=>{
                result.push(PriorityVal::new(Priority::Lev2, lexer::Token::DIV));
            }
        }
    }

    result
}


pub fn format_prival(mut from: Vec<PriorityVal>) -> Vec<PriorityVal> {
    // 5 + 8 * 4 - 7
    let mut result = Vec::new();
    let mut count = 0;
    while count < from.len() {
        if *from[count].get_level() == Priority::Lev2 {
            if count != 0 {
                from[count - 1].set_level(Priority::Lev2);
            }
            if count > 1 {
                from[count - 2].set_level(Priority::Lev2);
            }
            if count != from.len() - 1 {
                from[count + 1].set_level(Priority::Lev2);
            }
        }
        count += 1;
    }

    result
}

fn howlong_lev2(target: &Vec<PriorityVal>, idx: usize) -> usize {
    let mut count = idx;
    let mut result = 0;
    while count < target.len() {
        if *target[count].get_level() != Priority::Lev2 {
            return result;
        }
        result += 1
    }
    return result;
}

pub fn resolve_prival(mut from: Vec<PriorityVal>) -> Vec<PriorityVal> {
    let a = from.split(|x| *x.get_level() != Priority::Lev2);
    
    from
    
    //let mut result=Vec::new();
    /*let mut count=from.len()-1;
    loop {
        match from[count].get_level() {
            parser::Priority::Lev2=>{
                let len=howlong_lev2(&from, count);
                from.split
            }
            _=>{
                //do nothing
                // if let ?
            }
        }



        if count==0{
            break;
        }
        count-=1;
    }
    result
    */
}